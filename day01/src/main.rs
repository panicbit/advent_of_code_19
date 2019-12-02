#[macro_use] extern crate aoc;

#[aoc(2019, 01, 1)]
fn main(input: &str) -> f32 {
    input
    .lines()
    .map(|s| {
        let mass = s.parse::<f32>().unwrap();
        let fuel = (mass / 3.).floor() - 2.;
        fuel
    })
    .sum()
}
