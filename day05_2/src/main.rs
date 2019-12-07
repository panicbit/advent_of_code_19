#[macro_use] extern crate aoc;

#[aoc(2019, 05, 2)]
fn main(input: &str) -> isize {
    let mem = input
        .trim()
        .split(',')
        .map(|cell| cell.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    let mut vm = intcode::VM::new(mem);
    vm.set_inputs(vec![5]);
    vm.run();

    let outputs = vm.outputs();

    *outputs.last().unwrap()
}
