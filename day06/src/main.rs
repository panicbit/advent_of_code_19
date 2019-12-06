#[macro_use] extern crate aoc;

use std::collections::HashMap;

#[aoc(2019, 06, 1)]
fn main(input: &str) -> usize {
    let map = parse_orbit_map(input);
    let mut num_orbits = 0;

    traverse_orbit_map(&map, "COM", |orbits, object| {
        println!("{:?} ) {:?}", orbits, object);
        num_orbits += orbits.len();
    });

    num_orbits
}

fn traverse_orbit_map<F>(map: &HashMap<&str, Vec<&str>>, start: &str, mut f: F)
where
    F: FnMut(&[&str], &str),
{
    let mut orbiting = Vec::new();

    recur(map, start, &mut f, &mut orbiting);

    fn recur<'a, F>(map: &HashMap<&str, Vec<&'a str>>, start: &'a str, f: &mut F, orbiting: &mut Vec<&'a str>)
    where
        F: FnMut(&[&str], &str),
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
