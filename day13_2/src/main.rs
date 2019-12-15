#[macro_use] extern crate aoc;

use std::collections::HashMap;
use intcode::Op;
use std::cmp::Ordering;
use termion::{clear, cursor};

#[aoc(2019, 13, 2)]
fn main(input: &str) -> isize {
    let mut mem = intcode::parse(input);

    // free play
    mem[0] = 2;

    let mut vm = intcode::VM::new(mem);
    let mut xy = (None, None);
    let mut field = HashMap::new();
    let mut score = 0;
    let mut ball_x = 0;
    let mut paddle_x = 0;

    vm.run_tracing(|vm, prev, next| {
        if prev.op() == Op::WriteOutput {
            let value = *vm.outputs().last().unwrap();

            xy = match xy {
                (None, None) => (Some(value), None),
                (Some(x), None) => (Some(x), Some(value)),
                (Some(-1), Some(0)) => {
                    score = value;
                    (None, None)
                },
                (Some(x), Some(y)) => {
                    // paddle
                    if value == 3 {
                        paddle_x = x;
                    }

                    // ball
                    if value == 4 {
                        ball_x = x;
                    }

                    field.insert((x, y), value);
                    (None, None)
                },
                _ => unreachable!()
            }
        }

        if next.op() == Op::ReadInput {
            render_field(&field, score);

            let joy = match paddle_x.cmp(&ball_x) {
                Ordering::Less => 1,
                Ordering::Greater => -1,
                Ordering::Equal => 0,
            };

            vm.queue_input(joy);
        }
    });

    vm.run();

    score
}

fn render_field(field: &HashMap<(isize, isize), isize>, score: isize) {
    let min_x = *field.keys().map(|(x, _)| x).min().unwrap();
    let max_x = *field.keys().map(|(x, _)| x).max().unwrap();
    let min_y = *field.keys().map(|(_, y)| y).min().unwrap();
    let max_y = *field.keys().map(|(_, y)| y).max().unwrap();

    print!("{}", clear::All);
    print!("{}", cursor::Goto(1, 1));

    println!("Score: {}", score);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let sym = match field.get(&(x, y)).unwrap_or(&0) {
                0 => " ",
                1 => "█",
                2 => "▒",
                3 => "▔",
                4 => "◯",
                _ => unreachable!(),
            };

            print!("{}", sym);
        }
        println!();
    }
}
