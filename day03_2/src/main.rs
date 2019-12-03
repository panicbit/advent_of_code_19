#[macro_use] extern crate aoc;

use std::collections::{HashMap, HashSet};

#[aoc(2019, 03, 2)]
fn main(input: &str) -> i32 {
    let wires = input
        .lines()
        .enumerate()
        .map(|(id, wire)| {
            wire
            .split(",")
            .map(|part| {
                let dir = &part[0..1];
                let len = part[1..].parse::<i32>().unwrap();
                Part { id, dir, len }
            })
            .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut trace = HashMap::<(i32, i32), Cell>::new();

    for wire in wires {
        let mut x = 0;
        let mut y = 0;
        let mut distance = 0;

        for part in wire {
            for _ in 0..part.len {
                match part.dir {
                    "R" => x += 1,
                    "L" => x -= 1,
                    "U" => y += 1,
                    "D" => y -= 1,
                    _ => unreachable!(),
                }

                distance += 1;

                let cell = trace
                    .entry((x, y))
                    .or_default();

                cell.crossings.insert(part.id);
                cell
                    .distance.entry(part.id)
                    .or_insert(distance);
            }
        }
    }

    let min_steps = trace
        .into_iter()
        .filter(|(_, cell)| cell.crossings.len() > 1)
        .map(|(_, cell)| cell.distance.values().sum::<i32>())
        .min()
        .unwrap();

    min_steps
}

struct Part<'a> {
    id: usize,
    len: i32,
    dir: &'a str,
}

#[derive(Default)]
struct Cell {
    crossings: HashSet<usize>,
    distance: HashMap<usize, i32>,
}
