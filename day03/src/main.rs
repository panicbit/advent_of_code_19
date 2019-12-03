#[macro_use] extern crate aoc;

use std::collections::{HashMap, HashSet};

#[aoc(2019, 03, 1)]
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

    let mut trace = HashMap::<(i32, i32), HashSet<usize>>::new();

    for wire in wires {
        let mut x = 0;
        let mut y = 0;

        for part in wire {
            for _ in 0..part.len {
                match part.dir {
                    "R" => x += 1,
                    "L" => x -= 1,
                    "U" => y += 1,
                    "D" => y -= 1,
                    _ => unreachable!(),
                }

                trace
                    .entry((x, y))
                    .or_default()
                    .insert(part.id);
            }
        }
    }

    let min_crossing_distance = trace
        .into_iter()
        .filter(|(_, crossings)| crossings.len() > 1)
        .map(|((x, y), _)| x.abs() + y.abs())
        .min()
        .unwrap_or(0);

    min_crossing_distance
}

struct Part<'a> {
    id: usize,
    len: i32,
    dir: &'a str,
}
