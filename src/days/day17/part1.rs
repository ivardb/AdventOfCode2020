use std::collections::{HashSet, HashMap};
use crate::days::day17::{Point3d, parse_input3d, default_input};

pub fn run() {
    println!("{}", pocket_str(default_input()).unwrap())
}

pub fn pocket_str(input : &str) -> Result<usize, ()> {
    pocket(parse_input3d(input))
}

pub fn pocket(input : HashSet<Point3d>) -> Result<usize, ()> {
    let mut active = input;
    for _i in 0..6 {
        let mut neighbours: HashMap<Point3d, u8> = HashMap::new();
        for p in &active {
            for (dx, dy, dz) in [(0, 1, 0),(0, -1, 0),(1, 1, 0),(1, -1, 0),(1, 0, 0),(-1, 1, 0),(-1, 0, 0),(-1, -1, 0),
                (0, 1, 1),(0, -1, 1),(1, 1, 1),(1, -1, 1),(1, 0, 1),(-1, 1, 1),(-1, 0, 1),(-1, -1, 1), (0, 0, 1),
                (0, 1, -1),(0, -1, -1),(1, 1, -1),(1, -1, -1),(1, 0, -1),(-1, 1, -1),(-1, 0, -1),(-1, -1, -1), (0, 0, -1)].iter() {
                neighbours.entry(p.update(*dx, *dy, *dz)).and_modify(|c| *c += 1).or_insert(1);
            }
        }
        let mut new_active = HashSet::new();
        for (p, c) in neighbours {
            if c == 2 && active.contains(&p) {
                new_active.insert(p);
            } else if c == 3 {
                new_active.insert(p);
            }
        }
        active = new_active;
    }
    Ok(active.len())
}

#[cfg(tests)]
pub mod tests {
    use super::*;

    #[test]
    pub fn part1_answer() {
        assert_eq!(pocket_str(default_input()), 388)
    }
}