#[macro_use] extern crate aoc;

use itertools::Itertools;

#[aoc(2019, 08, 2)]
fn main(input: &str) -> i32 {
    println!("{}",
        input
        .chars()
        .map(|c| c.to_digit(10))
        .flatten()
        .chunks(25 * 6)
        .into_iter()
        .map(|row| row.collect_vec())
        .collect_vec()
        .into_iter()
        .rev()
        .fold1(|a, b| {
            a
            .into_iter()
            .zip(b)
            .map(|pixels| match pixels {
                (a, 2) => a,
                (_, b) => b,
            })
            .collect_vec()
        })
        .unwrap()
        .into_iter()
        .map(|pixel| match pixel {
            1 => '█',
            _ => ' ',
        })
        .chunks(25)
        .into_iter()
        .map(|mut row| row.join(""))
        .join("\n")
    );

    panic!("HUMAN REQUIRED");
}
