#[macro_use] extern crate aoc;

#[aoc(2019, 02, 1)]
fn main(input: &str) -> usize {
    let mut mem = input
        .trim()
        .split(',')
        .map(|cell| cell.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    // "Restore gravity assist program"
    mem[1] = 12;
    mem[2] = 2;

    let mut ip = 0;

    loop {
        let opcode = mem[ip];
        let a = mem[ip + 1];
        let b = mem[ip + 2];
        let target = mem[ip + 3];

        match opcode {
            1 => mem[target] = mem[a] + mem[b],
            2 => mem[target] = mem[a] * mem[b],
            99 => break,
            code => unreachable!("code: {}", code),
        }

        ip += 4;
    }

    mem[0]
}
