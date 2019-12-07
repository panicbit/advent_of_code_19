#[macro_use] extern crate aoc;

#[aoc(2019, 05, 2)]
fn main(input: &str) -> isize {
    let mem = intcode::parse(input);

    let mut vm = intcode::VM::new(mem);
    vm.add_input(5);
    vm.run();

    let outputs = vm.outputs();

    *outputs.last().unwrap()
}
