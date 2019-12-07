#[macro_use] extern crate aoc;

#[aoc(2019, 02, 2)]
fn main(input: &str) -> isize {
    let mem = input
        .trim()
        .split(',')
        .map(|cell| cell.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut vm = intcode::VM::new(mem.clone());
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
