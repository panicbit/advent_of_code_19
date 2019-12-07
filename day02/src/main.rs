#[macro_use] extern crate aoc;

#[aoc(2019, 02, 1)]
fn main(input: &str) -> isize {
    let mem = intcode::parse(input);

    let mut vm = intcode::VM::new(mem);

    // "Restore gravity assist program"
    vm.write(1, 12);
    vm.write(2, 2);

    vm.run();

    vm.read(0)
}
