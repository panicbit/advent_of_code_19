#[macro_use] extern crate aoc;

use std::collections::{HashMap, VecDeque};

#[aoc(2019, 06, 2)]
fn main(input: &str) -> usize {
    let map = parse_orbit_map(input);

    let mut you_path = None;
    let mut santa_path = None;

    traverse_orbit_map(&map, "COM", |orbits, object| {
        match object {
            "YOU" => you_path = Some(VecDeque::from(orbits.to_vec())),
            "SAN" => santa_path = Some(VecDeque::from(orbits.to_vec())),
            _ => {},
        }
    });

    let mut you_path = you_path.expect("YOU not found");
    let mut santa_path = santa_path.expect("SAN not found");

    while you_path[0] == santa_path[0] {
        you_path.pop_front();
        santa_path.pop_front();
    }

    you_path.len() + santa_path.len()
}

fn traverse_orbit_map<'a, F>(map: &HashMap<&str, Vec<&'a str>>, start: &'a str, mut f: F)
where
    F: FnMut(&[&'a str], &'a str),
{
    let mut orbiting = Vec::new();

    recur(map, start, &mut f, &mut orbiting);

    fn recur<'a, F>(map: &HashMap<&str, Vec<&'a str>>, start: &'a str, f: &mut F, orbiting: &mut Vec<&'a str>)
    where
        F: FnMut(&[&'a str], &'a str),
    {
        f(orbiting, start);
        orbiting.push(start);

        for object in map.get(start).unwrap_or(&vec![]) {
            recur(map, object, f, orbiting);
        }

        orbiting.pop();
    }

}

fn parse_orbit_map(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut orbit_map = HashMap::new();

    for orbit in input.lines() {
        let mut orbit = orbit.split(')');
        let orbited = orbit.next().unwrap();
        let orbiter = orbit.next().unwrap();
        orbit_map
            .entry(orbited)
            .or_insert(vec![])
            .push(orbiter);
    }

    orbit_map
}
