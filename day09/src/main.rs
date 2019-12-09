#[macro_use] extern crate aoc;

#[aoc(2019, 09, 1)]
fn main(input: &str) -> isize {
    let mem = intcode::parse(input);

    let mut vm = intcode::VM::new(mem);
    vm.add_input(1);
    vm.run();

    vm.outputs()[0]
}
