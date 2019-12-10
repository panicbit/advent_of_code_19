#[macro_use] extern crate aoc;

use itertools::Itertools;
use std::f64::consts::PI;

#[aoc(2019, 10, 2)]
fn main(input: &str) -> f64 {
// let input = ".#....#####...#..
// ##...##.#####..##
// ##...#...#.#####.
// ..#.....#...###..
// ..#.#.....#....##";
    let mut asteroids: Vec<Asteroid> =
        input
        .lines()
        .enumerate()
        .flat_map(|(y, row)|
            row
            .chars()
            .enumerate()
            .filter(|&(_, c)| c == '#')
            .map(move |(x, _)| Asteroid {
                x: x as f64,
                y: y as f64,
            })
        )
        .collect();

    let (index, base) =
        asteroids
        .iter()
        .cloned()
        .enumerate()
        .max_by_key(|(_, asteroid)| asteroid.count_visible(&asteroids))
        .unwrap();

    asteroids.remove(index);
    asteroids.sort_by(|a, b| {
        let direction = base.direction(a).partial_cmp(&base.direction(b)).unwrap();
        let distance = base.distance(a).partial_cmp(&base.distance(b)).unwrap();
        direction.then(distance)
    });

    let mut next_direction = 0.0;
    let mut last_destroyed_asteroid = None;
    let mut num_destroyed = 0;
    while num_destroyed < 200 {
        let next = asteroids
            .iter()
            .enumerate()
            .find(|(_, asteroid)| base.direction(asteroid) >= next_direction);

        let (index, asteroid) = match next {
            Some(next) => next,
            None => {
                next_direction = 0.0;
                continue;
            }
        };

        let current_direction = base.direction(asteroid);

        next_direction =
            asteroids.iter()
            .map(|asteroid| base.direction(asteroid))
            .filter(|&direction| direction > current_direction)
            .min_by(|a, b| a.partial_cmp(&b).unwrap())
            .unwrap_or(0.0);

        last_destroyed_asteroid = Some(asteroid.clone());

        asteroids.remove(index);

        num_destroyed += 1;
    }

    let asteroid = last_destroyed_asteroid.unwrap();

    asteroid.x * 100. + asteroid.y
}

#[derive(Debug, PartialEq, Clone)]
struct Asteroid {
    x: f64,
    y: f64,
}

impl Asteroid {
    fn direction(&self, other: &Asteroid) -> f64 {
        let x = other.x - self.x;
        let y = self.y - other.y;

        let angle = x.atan2(y);

        if angle >= 0. {
            angle * (180. / PI)
        } else {
            angle * (180. / PI) + 360.
        }
    }

    fn distance(&self, other: &Asteroid) -> f64 {
        let x = self.x - other.x;
        let y = self.y - other.y;

        (x * x + y * y).sqrt()
    }

    fn count_visible(&self, other: &[Asteroid]) -> usize {
        other
            .iter()
            .filter(|&asteroid| self != asteroid)
            .unique_by(|asteroid| self.direction(asteroid).to_string())
            .count()
    }
}
