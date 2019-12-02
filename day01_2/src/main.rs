#[macro_use] extern crate aoc;

use std::iter::successors;

#[aoc(2019, 01, 2)]
fn main(input: &str) -> f32 {
    input
    .lines()
    .map(|s| {
        let mass = s.parse::<f32>().unwrap();
        let fuel = successors(Some(mass), |mass| Some((mass / 3.).floor() - 2.))
            .skip(1)
            .take_while(|&fuel| fuel > 0.)
            .sum::<f32>();
        fuel
    })
    .sum()
}
