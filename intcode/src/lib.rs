use std::sync::mpsc::{channel, Sender, Receiver};

use std::collections::VecDeque;

pub fn parse(input: &str) -> Vec<isize> {
    input
    .trim()
    .split(',')
    .map(|cell| cell.parse::<isize>().unwrap())
    .collect()
}

pub struct VM<'a, Context = ()> {
    mem: Vec<isize>,
    ip: usize,
    relative_base: isize,
    input_queue: VecDeque<isize>,
    input_tx: Sender<isize>,
    input_rx: Receiver<isize>,
    output_tx: Option<Sender<isize>>,
    outputs: Vec<isize>,
    input_provider: Option<Box<dyn FnMut(&mut Context) -> isize + Send + 'a>>,
    on_output: Option<Box<dyn FnMut(&mut Context, isize) + Send + 'a>>,
    debug: bool,
    did_run: bool,
    context: Context,
}

impl VM<'_, ()> {
    pub fn new(mem: impl Into<Vec<isize>>) -> Self {
        Self::with_context(mem, ())
    }
}

impl<'a, Context> VM<'a, Context> {
    pub fn with_context(mem: impl Into<Vec<isize>>, context: Context) -> Self {
        let (input_tx, input_rx) = channel();

        Self {
            mem: mem.into(),
            ip: 0,
            relative_base: 0,
            input_queue: VecDeque::new(),
            input_tx,
            input_rx,
            output_tx: None,
            outputs: vec![],
            input_provider: None,
            on_output: None,
            debug: false,
            did_run: false,
            context,
        }
    }

    pub fn set_on_output(&mut self, f: impl FnMut(&mut Context, isize) + Send + 'a) {
        self.on_output = Some(Box::new(f));
    }

    pub fn set_input_provider(&mut self, f: impl FnMut(&mut Context, ) -> isize + Send + 'a) {
        self.input_provider = Some(Box::new(f));
    }

    pub fn add_input(&mut self, value: isize) {
        self.input_tx.send(value).ok();
    }

    pub fn queue_input(&mut self, value: isize) {
        self.input_queue.push_back(value);
    }

    pub fn set_output(&mut self, output_tx: Sender<isize>) {
        self.output_tx = Some(output_tx);
    }

    pub fn input(&self) -> Sender<isize> {
        self.input_tx.clone()
    }

    pub fn set_debug(&mut self, state: bool) {
        self.debug = state;
    }

    pub fn read(&self, addr: usize) -> isize {
        let value = self.mem.get(addr).copied().unwrap_or(0);

        if self.debug {
            println!("read({}) -> {}", addr, value);
        }

        value
    }

    pub fn write(&mut self, addr: usize, value: isize) {
        if self.debug {
            println!("write({}) -> {}", addr, value);
        }

        self.ensure_mem_size(addr + 1);
        self.mem[addr] = value;
    }

    pub fn outputs(&self) -> &[isize] {
        &self.outputs
    }

    pub fn last_output(&self) -> Option<isize> {
        self.outputs.last().copied()
    }

    fn code(&self) -> isize {
        self.read(self.ip)
    }

    pub fn next_op_code(&self) -> OpCode {
        OpCode::parse(self.code())
    }

    fn read_arg(&self, index: usize, modes: &[Mode]) -> isize {
        assert!(index > 0);

        let addr = match modes.get(index - 1).unwrap_or(&Mode::Position) {
            Mode::Position => self.read(self.ip + index),
            Mode::Immediate => (self.ip + index) as isize,
            Mode::Relative => {
                let offset = self.read(self.ip + index);
                self.relative_base + offset
            }
        };

        self.read(addr as usize)
    }

    fn write_arg(&mut self, index: usize, value: isize, modes: &[Mode]) {
        assert!(index > 0);

        let arg = self.read(self.ip + index);

        let addr = match modes.get(index -1).unwrap_or(&Mode::Position) {
            Mode::Position | Mode::Immediate => arg,
            Mode::Relative => self.relative_base + arg,
        };

        self.write(addr as usize, value);
    }

    pub fn run(&mut self) {
        while !self.step().is_halt() {}
    }

    pub fn run_tracing(&mut self, mut tracer: impl FnMut(&mut Self, OpCode, &OpCode)) {
        let mut previous_op_code = OpCode {
            op: Op::Halt,
            modes: vec![],
        };

        loop {
            let next_op_code = self.next_op_code();

            tracer(self, previous_op_code, &next_op_code);
            self.execute(&next_op_code);

            if next_op_code.is_halt() {
                return;
            }

            previous_op_code = next_op_code;
        }
    }

    pub fn step(&mut self) -> OpCode {
        let next_op_code = self.next_op_code();
        self.execute(&next_op_code);
        next_op_code
    }

    pub fn execute(&mut self, op_code: &OpCode) {
        self.did_run = true;

        let modes = &op_code.modes;

        if self.debug {
            println!("{:?}", op_code);
        }

        match op_code.op {
            Op::Add => self.op_add(modes),
            Op::Mul => self.op_mul(modes),
            Op::ReadInput => self.op_read_input(modes),
            Op::WriteOutput => self.op_write_output(modes),
            Op::JumpIfTrue => self.op_jump_if_true(modes),
            Op::JumpIfFalse => self.op_jump_if_false(modes),
            Op::LessThan => self.op_less_than(modes),
            Op::Equals => self.op_equals(modes),
            Op::AdjustRelativeBase => self.op_adjust_relative_base(modes),
            Op::Halt => {},
        }
    }

    fn ensure_mem_size(&mut self, size: usize) {
        while self.mem.len() < size {
            self.mem.push(0);
        }
    }

    fn op_add(&mut self, modes: &[Mode]) {
        let a = self.read_arg(1, modes);
        let b = self.read_arg(2, modes);
        self.write_arg(3, a + b, modes);
        self.ip += 4;
    }

    fn op_mul(&mut self, modes: &[Mode]) {
        let a = self.read_arg(1, modes);
        let b = self.read_arg(2, modes);
        self.write_arg(3, a * b, modes);
        self.ip += 4;
    }

    fn op_read_input(&mut self, modes: &[Mode]) {
        let queued_input = self.input_queue.pop_front();

        let value = match (queued_input, &mut self.input_provider) {
            (Some(queued_input), _) => queued_input,
            (_, Some(input_provider)) => input_provider(&mut self.context),
            _ => self.input_rx.recv().expect("failed to read value"),
        };

        self.write_arg(1, value, modes);
        self.ip += 2;
    }

    fn op_write_output(&mut self, modes: &[Mode]) {
        let value = self.read_arg(1, modes);

        if self.debug {
            println!("Output: {}", value);
        }

        if let Some(on_output) = &mut self.on_output {
            on_output(&mut self.context, value);
        }

        if let Some(output_tx) = &self.output_tx {
            output_tx.send(value).ok();
        }

        self.outputs.push(value);
        self.ip += 2;
    }

    fn op_jump_if_true(&mut self, modes: &[Mode]) {
        let cond = self.read_arg(1, modes);
        let ip = self.read_arg(2, modes);

        if cond != 0 {
            self.ip = ip as usize;
        } else {
            self.ip += 3;
        }
    }

    fn op_jump_if_false(&mut self, modes: &[Mode]) {
        let cond = self.read_arg(1, modes);
        let ip = self.read_arg(2, modes);

        if cond == 0 {
            self.ip = ip as usize;
        } else {
            self.ip += 3;
        }
    }

    fn op_less_than(&mut self, modes: &[Mode]) {
        let a = self.read_arg(1, modes);
        let b = self.read_arg(2, modes);
        let value = (a < b) as isize;

        self.write_arg(3, value, modes);
        self.ip += 4;
    }

    fn op_equals(&mut self, modes: &[Mode]) {
        let a = self.read_arg(1, modes);
        let b = self.read_arg(2, modes);
        let value = (a == b) as isize;

        self.write_arg(3, value, modes);
        self.ip += 4;
    }

    fn op_adjust_relative_base(&mut self, modes: &[Mode]) {
        let adjustment = self.read_arg(1, modes);
        self.relative_base += adjustment;

        if self.debug {
            println!("new relative base: {}", self.relative_base);
        }

        self.ip += 2;
    }
}

#[derive(Debug)]
pub struct OpCode {
    op: Op,
    modes: Vec<Mode>,
}

impl OpCode {
    fn parse(mut code: isize) -> Self {
        let op = match code % 100 {
            1 => Op::Add,
            2 => Op::Mul,
            3 => Op::ReadInput,
            4 => Op::WriteOutput,
            5 => Op::JumpIfTrue,
            6 => Op::JumpIfFalse,
            7 => Op::LessThan,
            8 => Op::Equals,
            9 => Op::AdjustRelativeBase,
            99 => Op::Halt,
            code => unreachable!("code: {}", code),
        };
        code /= 100;

        let mut modes = Vec::new();

        while code > 0 {
            let mode = match code % 10 {
                0 => Mode::Position,
                1 => Mode::Immediate,
                2 => Mode::Relative,
                mode => unreachable!("mode: {}", mode),
            };

            modes.push(mode);
            code /= 10;
        }

        OpCode {
            op, modes
        }
    }

    pub fn op(&self) -> Op {
        self.op
    }

    pub fn modes(&self) -> &[Mode] {
        &self.modes
    }

    pub fn is_halt(&self) -> bool {
        self.op == Op::Halt
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Op {
    Add,
    Mul,
    ReadInput,
    WriteOutput,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    AdjustRelativeBase,
    Halt,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Mode {
    Position,
    Immediate,
    Relative,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn halt() {
        let mut vm = VM::new(vec![99]);
        vm.run();
    }

    #[test]
    fn add_immediate() {
        let mut vm = VM::new(vec![11_01, 2, 3, 0, 99]);
        vm.run();
        assert_eq!(vm.read(0), 5);
    }

    #[test]
    fn read_input() {
        let mut vm = VM::new(vec![3, 0, 99]);
        vm.add_input(42);
        vm.run();
        assert_eq!(vm.read(0), 42);
    }

    #[test]
    fn modes() {
        let op_code = OpCode::parse(201_01);

        assert_eq!(op_code.modes, [
            Mode::Immediate,
            Mode::Position,
            Mode::Relative,
        ]);
    }
}

impl<Context> Drop for VM<'_, Context> {
    fn drop(&mut self) {
        if !self.did_run {
            eprintln!("WARNING: VM did not run");
        }
    }
}
