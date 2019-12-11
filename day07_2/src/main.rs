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
    let (tx, rx) = channel();
    let rx = phases
        .into_iter()
        .rev()
        .fold(rx, |rx, phase| {
            let (tx, new_rx) = channel();

            let mut vm = VM::new(mem);

            vm.queue_input(phase);

            vm.set_input_provider(move |_| {
                let signal = rx.recv().unwrap();
                signal
            });

            vm.set_on_output(move |_, signal| {
                tx.send(signal).ok();
            });

            thread::spawn(move || vm.run());

            new_rx
        });

    // send initial signal to first stage
    tx.send(0).ok();

    let mut signal = 0;
    while let Ok(new_signal) = rx.recv() {
        signal = new_signal;
        tx.send(new_signal).ok();
    }

    signal
}
