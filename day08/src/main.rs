#[macro_use] extern crate aoc;

use itertools::Itertools;

#[aoc(2019, 08, 1)]
fn main(input: &str) -> usize {
    input
    .chars()
    .map(|c| c.to_digit(10))
    .flatten()
    .chunks(25 * 6)
    .into_iter()
    .map(|layer| layer.collect_vec())
    .collect_vec()
    .into_iter()
    .min_by_key(|layer|
        layer
        .iter()
        .filter(|&&pixel| pixel == 0)
        .count()
    )
    .map(|layer| {
        layer
        .iter()
        .filter(|&&pixel| pixel == 1)
        .count()
        *
        layer
        .iter()
        .filter(|&&pixel| pixel == 2)
        .count()
    })
    .unwrap()
}
