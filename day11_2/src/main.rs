#[macro_use] extern crate aoc;

use std::collections::HashMap;
use intcode::{VM, Op};

#[aoc(2019, 11, 2)]
fn main(input: &str) -> usize {
    let mem = intcode::parse(input);
    let mut vm = VM::new(mem);

    let mut hull = HashMap::new();
    let mut direction = Direction::Up;
    let mut x = 0;
    let mut y = 0;
    let mut next_output = NextOutput::Color;

    hull.insert((0, 0), true);

    vm.run_tracing(|vm, prev, next| {
        if prev.op() == Op::WriteOutput {
            let output = vm.outputs().last().unwrap();

            match next_output {
                NextOutput::Color => {
                    let is_white = match output {
                        0 => false,
                        1 => true,
                        _ => panic!("invalid color {}", output),
                    };

                    hull.insert((x, y), is_white);
                    next_output = NextOutput::Direction;
                },
                NextOutput::Direction => {
                    match output {
                        0 => direction.turn_left(),
                        1 => direction.turn_right(),
                        _ => panic!("invalid turn {}", output)
                    }

                    direction.go(&mut x, &mut y);
                    next_output = NextOutput::Color;
                },
            }
        }

        if next.op() == Op::ReadInput {
            let color = hull.get(&(x, y)).copied().unwrap_or(false) as isize;
            vm.add_input(color);
        }
    });

    let min_x = hull.keys().map(|(x, _)| *x).min().unwrap_or(0);
    let max_x = hull.keys().map(|(x, _)| *x).max().unwrap_or(0);
    let min_y = hull.keys().map(|(_, y)| *y).min().unwrap_or(0);
    let max_y = hull.keys().map(|(_, y)| *y).max().unwrap_or(0);

    for y in (min_y ..= max_y).rev() {
        for x in min_x ..= max_x {
            let is_white = hull.get(&(x, y)).copied().unwrap_or(false);

            if is_white {
                print!("█");
            } else {
                print!(" ");
            }
        }

        println!();
    }

    panic!("NEED HUMAN HELP")
}

enum NextOutput {
    Color,
    Direction,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_left(&mut self) {
        *self = match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(&mut self) {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn go(&self, x: &mut i32, y: &mut i32) {
        match self {
            Direction::Up => *y += 1,
            Direction::Down => *y -= 1,
            Direction::Left => *x -= 1,
            Direction::Right => *x += 1,
        }
    }
}
