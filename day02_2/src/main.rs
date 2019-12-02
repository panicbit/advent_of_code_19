#[macro_use] extern crate aoc;

#[aoc(2019, 02, 2)]
fn main(input: &str) -> usize {
    let mem = input
        .trim()
        .split(',')
        .map(|cell| cell.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut vm = VM::new(mem);

    // "Restore gravity assist program"
    vm.write(1, 12);
    vm.write(2, 2);

    vm.run();

    vm.read(0)
}

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

    fn opcode(&self) -> usize {
        self.read(self.ip)
    }

    fn read_3_args(&self) -> (usize, usize, usize) {
        (
            self.read(self.ip + 1),
            self.read(self.ip + 2),
            self.read(self.ip + 3),
        )
    }

    fn run(&mut self) {
        loop {
            match self.opcode() {
                1 => self.op_add(),
                2 => self.op_mul(),
                99 => break,
                code => unreachable!("code: {}", code),
            }
        }
    }

    fn op_add(&mut self) {
        let (addr_a, addr_b, target) = self.read_3_args();
        let a = self.read(addr_a);
        let b = self.read(addr_b);
        self.write(target, a + b);
        self.ip += 4;
    }

    fn op_mul(&mut self) {
        let (addr_a, addr_b, target) = self.read_3_args();
        let a = self.read(addr_a);
        let b = self.read(addr_b);
        self.write(target, a * b);
        self.ip += 4;
    }
}
