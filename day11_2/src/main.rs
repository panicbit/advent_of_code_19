#[macro_use] extern crate aoc;

use std::collections::HashMap;
use std::cell::RefCell;

#[aoc(2019, 11, 2)]
fn main(input: &str) -> usize {
    
    let state = RefCell::new(State {
        hull: HashMap::new(),
        next_input: NextInput::Color,
        direction: Direction::Up,
        x: 0,
        y: 0,
    });

    state.borrow_mut().hull.insert((0, 0), true);
    
    let mem = intcode::parse(input);
    let mut vm = intcode::VM::new(mem);

    vm.set_on_output(|output| {
        let state = &mut *state.borrow_mut();

        match state.next_input {
            NextInput::Color => {
                let is_white = match output {
                    0 => false,
                    1 => true,
                    _ => panic!("invalid color {}", output),
                };

                state.hull.insert((state.x, state.y), is_white);
                state.next_input = NextInput::Direction;
            },
            NextInput::Direction => {
                match output {
                    0 => state.direction.turn_left(),
                    1 => state.direction.turn_right(),
                    _ => panic!("invalid turn {}", output)
                }

                state.direction.go(&mut state.x, &mut state.y);
                state.next_input = NextInput::Color;
            }
        }

    });

    vm.set_input_provider(|| {
        let state = state.borrow();
        state.hull.get(&(state.x, state.y))
            .cloned()
            .unwrap_or(false) as isize
    });

    vm.run();

    let state = state.borrow();

    let min_x = state.hull.keys().map(|(x, _)| *x).min().unwrap_or(0);
    let max_x = state.hull.keys().map(|(x, _)| *x).max().unwrap_or(0);
    let min_y = state.hull.keys().map(|(_, y)| *y).min().unwrap_or(0);
    let max_y = state.hull.keys().map(|(_, y)| *y).max().unwrap_or(0);

    for y in (min_y ..= max_y).rev() {
        for x in min_x ..= max_x {
            let is_white = state.hull.get(&(x, y)).copied().unwrap_or(false);

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

struct State {
    hull: HashMap<(i32, i32), bool>,
    next_input: NextInput,
    direction: Direction,
    x: i32,
    y: i32,
}

#[derive(Copy,Clone)]
enum NextInput {
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
