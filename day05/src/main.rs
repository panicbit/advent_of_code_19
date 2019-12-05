#[macro_use] extern crate aoc;

#[aoc(2019, 05, 1)]
fn main(input: &str) -> usize {
    let mem = input
        .trim()
        .split(',')
        .map(|cell| cell.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut vm = VM::new(mem.clone());
            vm.write(1, noun);
            vm.write(2, verb);

            vm.run();

            if vm.read(0) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    unreachable!()
}

#[derive(Clone)]
struct VM {
    mem: Vec<usize>,
    ip: usize,
}

impl VM {
    fn new(mem: Vec<usize>) -> Self {
        Self {
            mem,
            ip: 0,
        }
    }

    fn read(&self, addr: usize) -> usize {
        self.mem[addr]
    }

    fn write(&mut self, addr: usize, value: usize) {
        self.mem[addr] = value;
    }

    fn code(&self) -> usize {
        self.read(self.ip)
    }

    fn op_code(&self) -> OpCode {
        OpCode::parse(self.code())
    }

    fn read_arg(&self, index: usize, mode: impl Into<Option<Mode>>) -> usize {
        let mode = mode.into().unwrap_or(Mode::Position);

        match mode {
            Mode::Immediate => self.read(self.ip + index),
            Mode::Position => {
                let position = self.read(self.ip + index);
                self.read(position)
            }
        }
    }

    fn read_3_args(&self, modes: &[Mode]) -> (usize, usize, usize) {
        (
            self.read_arg(0, modes.get(0).copied()),
            self.read_arg(1, modes.get(1).copied()),
            self.read_arg(2, modes.get(2).copied()),
        )
    }

    fn run(&mut self) {
        loop {
            let op_code = self.op_code();

            match op_code.op {
                Op::Add => self.op_add(&op_code.modes),
                Op::Mul => self.op_mul(&op_code.modes),
                Op::Halt => break,
            }
        }
    }

    fn op_add(&mut self, modes: &[Mode]) {
        let (addr_a, addr_b, target) = self.read_3_args(modes);
        let a = self.read(addr_a);
        let b = self.read(addr_b);
        self.write(target, a + b);
        self.ip += 4;
    }

    fn op_mul(&mut self, modes: &[Mode]) {
        let (addr_a, addr_b, target) = self.read_3_args(modes);
        let a = self.read(addr_a);
        let b = self.read(addr_b);
        self.write(target, a * b);
        self.ip += 4;
    }
}

struct OpCode {
    op: Op,
    modes: Vec<Mode>,
}

impl OpCode {
    fn parse(mut code: usize) -> Self {
        let op = match code % 100 {
            1 => Op::Add,
            2 => Op::Mul,
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

enum Op {
    Add,
    Mul,
    Halt,
}

#[derive(Copy, Clone)]
enum Mode {
    Position,
    Immediate,
}
