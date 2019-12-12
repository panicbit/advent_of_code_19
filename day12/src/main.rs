#[macro_use] extern crate aoc;

use regex::Regex;
use itertools::Itertools;
use std::cmp::Ordering;
use std::cell::RefCell;

#[aoc(2019, 12, 1)]
fn main(input: &str) -> i32 {
    let bodies =
        input
        .lines()
        .map(Vec3::parse)
        .map(Body::new)
        .map(RefCell::new)
        .collect_vec();

    for _ in 0..1000 {
        for (x, y) in bodies.iter().tuple_combinations() {
            let mut x = x.borrow_mut();
            let mut y = y.borrow_mut();
            x.apply_gravity(&y);
            y.apply_gravity(&x);
        }

        for body in &bodies {
            let mut body = body.borrow_mut();
            body.apply_velocity();
        }
    }

    bodies
    .iter()
    .map(|body| body.borrow().total_energy())
    .sum()
}

#[derive(Debug, Copy, Clone)]
struct Body {
    position: Vec3,
    velocity: Vec3,
}

impl Body {
    fn new(position: Vec3) -> Self {
        Self {
            position,
            velocity: Vec3 {
                x: 0,
                y: 0,
                z: 0,
            },
        }
    }

    fn apply_gravity(&mut self, other: &Self) {
        self.velocity.x += calc_gravity(self.position.x, other.position.x);
        self.velocity.y += calc_gravity(self.position.y, other.position.y);
        self.velocity.z += calc_gravity(self.position.z, other.position.z);
    }

    fn potential_energy(&self) -> i32 {
        self.position.x.abs() +
        self.position.y.abs() +
        self.position.z.abs()
    }

    fn kinetic_energy(&self) -> i32 {
        self.velocity.x.abs() +
        self.velocity.y.abs() +
        self.velocity.z.abs()
    }

    fn total_energy(&self) -> i32 {
        self.potential_energy() * self.kinetic_energy()
    }

    fn apply_velocity(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }
}

fn calc_gravity(pos1: i32, pos2: i32) -> i32 {
    match pos1.cmp(&pos2) {
        Ordering::Less => 1,
        Ordering::Greater => -1,
        Ordering::Equal => 0,
    }
}

#[derive(Debug, Copy, Clone)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vec3 {
    fn parse(s: &str) -> Self {
        let re = Regex::new(r"<x=(.*), y=(.*), z=(.*)>").unwrap();
        let caps = re.captures(s).unwrap();

        Self {
            x: caps[1].parse().unwrap(),
            y: caps[2].parse().unwrap(),
            z: caps[3].parse().unwrap(),
        }
    }
}
