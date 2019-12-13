#[macro_use] extern crate aoc;

use std::collections::HashMap;

#[aoc(2019, 13, 1)]
fn main(input: &str) -> usize {
    let mem = intcode::parse(input);
    let mut vm = intcode::VM::new(mem);

    vm.run();

    let mut field = HashMap::new();

    for tile in vm.outputs().chunks(3) {
        let x = tile[0];
        let y = tile[1];
        let id = tile[2];

        field.insert((x, y), id);
    }

    field.values().filter(|&&id| id == 2).count()
}
