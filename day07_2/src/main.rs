#[macro_use] extern crate aoc;

use itertools::Itertools;
use intcode::VM;
use std::sync::mpsc::channel;
use std::thread;

#[aoc(2019, 07, 2)]
fn main(input: &str) -> isize {
    let mem = intcode::parse(input);

    (5..=9)
    .permutations(5)
    .map(|phases| run_series(&mem, phases))
    .max()
    .unwrap()
}

fn run_series(mem: &[isize], phases: Vec<isize>) -> isize {
    let mut stages: Vec<VM> = phases
        .into_iter()
        .map(|phase| {
            let mut vm = intcode::VM::new(mem);
            vm.add_input(phase);
            vm
        })
        .collect();

    // hook up stage inputs and outputs
    for i in 1..stages.len() {
        let input = stages[i].input();
        stages[i-1].set_output(input);
    }

    let first_stage = stages.first_mut().unwrap();
    let input_tx = first_stage.input();

    let (output_tx, output_rx) = channel();
    let last_stage = stages.last_mut().unwrap();
    last_stage.set_output(output_tx);

    // run stages
    for mut stage in stages {
        thread::spawn(move || stage.run());
    }

    // send initial signgal to first stage
    input_tx.send(0).ok();

    let mut signal = 0;
    while let Ok(new_signal) = output_rx.recv() {
        signal = new_signal;
        input_tx.send(new_signal).ok();
    }

    signal
}
