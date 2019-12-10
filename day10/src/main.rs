#[macro_use] extern crate aoc;

use itertools::Itertools;

#[aoc(2019, 10, 1)]
fn main(input: &str) -> usize {
    let asteroids: Vec<Asteroid> =
        input
        .lines()
        .enumerate()
        .flat_map(|(y, row)|
            row
            .chars()
            .enumerate()
            .filter(|&(_, c)| c == '#')
            .map(move |(x, _)| Asteroid {
                x: x as f32,
                y: y as f32,
            })
        )
        .collect();

    asteroids
    .iter()
    .map(|asteroid| asteroid.count_visible(&asteroids))
    .max()
    .unwrap_or(0)
}

#[derive(Debug, PartialEq)]
struct Asteroid {
    x: f32,
    y: f32,
}

impl Asteroid {
    fn direction(&self, other: &Asteroid) -> f32 {
        let x = other.x - self.x;
        let y = other.y - self.y;

        x.atan2(y)
    }

    fn count_visible(&self, other: &[Asteroid]) -> usize {
        other
            .iter()
            .filter(|&asteroid| self != asteroid)
            .unique_by(|asteroid| self.direction(asteroid).to_string())
            .count()
    }
}
