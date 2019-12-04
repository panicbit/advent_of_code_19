#[macro_use] extern crate aoc;

use itertools::Itertools;

#[aoc(2019, 04, 2)]
fn main(input: &str) -> usize {
    let mut input = input.trim().split('-');
    let start: u32 = input.next().unwrap().parse().unwrap();
    let end: u32 = input.next().unwrap().parse().unwrap();

    (start..=end)
    .filter(is_six_digit)
    .filter(two_adjacent_digits_are_same)
    .filter(digits_never_decrease)
    .count()
}

fn is_six_digit(n: &u32) -> bool {
    (100_000..1_000_000).contains(n)
}

fn two_adjacent_digits_are_same(n: &u32) -> bool {
    n
    .to_string()
    .chars()
    .group_by(|digit| *digit)
    .into_iter()
    .any(|(_, run)| run.count() == 2)
}

fn digits_never_decrease(n: &u32) -> bool {
    n
    .to_string()
    .chars()
    .tuple_windows()
    .all(|(a, b)| a <= b)
}
