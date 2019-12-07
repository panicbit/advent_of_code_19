use std::collections::VecDeque;

pub fn parse(input: &str) -> Vec<isize> {
    input
    .trim()
    .split(',')
    .map(|cell| cell.parse::<isize>().unwrap())
    .collect()
}

#[derive(Clone)]
pub struct VM {
    mem: Vec<isize>,
    ip: usize,
    inputs: VecDeque<isize>,
    outputs: Vec<isize>,
    debug: bool,
    did_run: bool,
}

impl VM {
    pub fn new(mem: impl Into<Vec<isize>>) -> Self {
        Self {
            mem: mem.into(),
            ip: 0,
            inputs: VecDeque::new(),
            outputs: vec![],
            debug: false,
            did_run: false,
        }
    }

    pub fn set_inputs(&mut self, inputs: impl Into<Vec<isize>>) {
        self.inputs = inputs.into().into();
    }

    pub fn add_input(&mut self, value: isize) {
        self.inputs.push_back(value);
    }

    pub fn set_debug(&mut self, state: bool) {
        self.debug = state;
    }

    pub fn read(&self, addr: usize) -> isize {
        self.mem[addr]
    }

    pub fn write(&mut self, addr: usize, value: isize) {
        self.mem[addr] = value;
    }

    pub fn outputs(&self) -> &[isize] {
        &self.outputs
    }

    fn code(&self) -> isize {
        self.read(self.ip)
    }

    fn op_code(&self) -> OpCode {
        OpCode::parse(self.code())
    }

    fn read_arg(&self, index: usize, modes: &[Mode]) -> isize {
        assert!(index > 0);

        match modes.get(index - 1).unwrap_or(&Mode::Position) {
            Mode::Position => {
                let addr = self.read(self.ip + index);
                self.read(addr as usize)
            },
            Mode::Immediate => self.read(self.ip + index),
        }
    }

    fn write_arg(&mut self, index: usize, value: isize) {
        let addr = self.read(self.ip + index);
        self.write(addr as usize, value);
    }

    pub fn run(&mut self) {
        self.did_run = true;

        loop {
            let op_code = self.op_code();
            let modes = &op_code.modes;

            if self.debug {
                println!("{:?}", op_code);
            }

            match op_code.op {
                Op::Add => self.op_add(modes),
                Op::Mul => self.op_mul(modes),
                Op::ReadInput => self.op_read_input(),
                Op::WriteOutput => self.op_write_output(modes),
                Op::JumpIfTrue => self.op_jump_if_true(modes),
                Op::JumpIfFalse => self.op_jump_if_false(modes),
                Op::LessThan => self.op_less_than(modes),
                Op::Equals => self.op_equals(modes),
                Op::Halt => break,
            }
        }
    }

    fn op_add(&mut self, modes: &[Mode]) {
        let a = self.read_arg(1, modes);
        let b = self.read_arg(2, modes);
        self.write_arg(3, a + b);
        self.ip += 4;
    }

    fn op_mul(&mut self, modes: &[Mode]) {
        let a = self.read_arg(1, modes);
        let b = self.read_arg(2, modes);
        self.write_arg(3, a * b);
        self.ip += 4;
    }

    fn op_read_input(&mut self) {
        let value = self.inputs.pop_front().unwrap();
        self.write_arg(1, value);
        self.ip += 2;
    }

    fn op_write_output(&mut self, modes: &[Mode]) {
        let value = self.read_arg(1, modes);

        if self.debug {
            println!("Output: {}", value);
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

        self.write_arg(3, value);
        self.ip += 4;
    }

    fn op_equals(&mut self, modes: &[Mode]) {
        let a = self.read_arg(1, modes);
        let b = self.read_arg(2, modes);
        let value = (a == b) as isize;

        self.write_arg(3, value);
        self.ip += 4;
    }
}

#[derive(Debug)]
struct OpCode {
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
            99 => Op::Halt,
            code => unreachable!("code: {}", code),
        };
        code /= 100;

        let mut modes = Vec::new();

        while code > 0 {
            let mode = match code % 10 {
                0 => Mode::Position,
                1 => Mode::Immediate,
                mode => unreachable!("mode: {}", mode),
            };

            modes.push(mode);
            code /= 10;
        }

        OpCode {
            op, modes
        }
    }
}

#[derive(Debug)]
enum Op {
    Add,
    Mul,
    ReadInput,
    WriteOutput,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Mode {
    Position,
    Immediate,
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
        vm.set_inputs(vec![42]);
        vm.run();
        assert_eq!(vm.read(0), 42);
    }

    #[test]
    fn modes() {
        let op_code = OpCode::parse(101_01);

        assert_eq!(op_code.modes, [
            Mode::Immediate,
            Mode::Position,
            Mode::Immediate,
        ]);
    }
}

impl Drop for VM {
    fn drop(&mut self) {
        if !self.did_run {
            eprintln!("WARNING: VM did not run");
        }
    }
}
