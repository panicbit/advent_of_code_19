#[macro_use] extern crate aoc;

use itertools::Itertools;

#[aoc(2019, 07, 1)]
fn main(input: &str) -> isize {
    let mem = intcode::parse(input);

    (0..=4)
    .permutations(5)
    .map(|phases| run_series(&mem, phases))
    .max()
    .unwrap()
}

fn run_series(mem: &[isize], phases: Vec<isize>) -> isize {
    let mut signal = 0;

    for phase in phases {
        signal = run_stage(mem.clone(), phase, signal);
    }

    signal
}

fn run_stage(mem: &[isize], phase: isize, signal: isize) -> isize {
    let mut vm = intcode::VM::new(mem);
    vm.add_input(phase);
    vm.add_input(signal);
    vm.run();
    vm.outputs()[0]
}
