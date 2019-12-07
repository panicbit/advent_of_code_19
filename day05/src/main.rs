#[macro_use] extern crate aoc;

#[aoc(2019, 05, 1)]
fn main(input: &str) -> isize {
    let mem = intcode::parse(input);

    let mut vm = intcode::VM::new(mem);
    vm.set_inputs(vec![1]);
    vm.run();

    let outputs = vm.outputs();

    *outputs.last().unwrap()
}
