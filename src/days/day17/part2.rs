use std::collections::{HashSet, HashMap};
use crate::days::day17::{Point4d, parse_input4d, default_input};

pub fn run() {
    println!("{}", pocket_str(default_input()).unwrap())
}

pub fn pocket_str(input : &str) -> Result<usize, ()> {
    pocket(parse_input4d(input))
}

pub fn pocket(input : HashSet<Point4d>) -> Result<usize, ()> {
    let mut active = input;
    for _i in 0..6 {
        let mut neighbours: HashMap<Point4d, u8> = HashMap::new();
        for p in &active {
            for (dx, dy, dz, dw) in [(0, 1, 0, 0),(0, -1, 0, 0),(1, 1, 0, 0),(1, -1, 0, 0),(1, 0, 0, 0),(-1, 1, 0, 0),(-1, 0, 0, 0),(-1, -1, 0, 0),
                (0, 1, 1, 0),(0, -1, 1, 0),(1, 1, 1, 0),(1, -1, 1, 0),(1, 0, 1, 0),(-1, 1, 1, 0),(-1, 0, 1, 0),(-1, -1, 1, 0), (0, 0, 1, 0),
                (0, 1, -1, 0),(0, -1, -1, 0),(1, 1, -1, 0),(1, -1, -1, 0),(1, 0, -1, 0),(-1, 1, -1, 0),(-1, 0, -1, 0),(-1, -1, -1, 0), (0, 0, -1, 0),
                (0, 1, 0, 1),(0, -1, 0, 1),(1, 1, 0, 1),(1, -1, 0, 1),(1, 0, 0, 1),(-1, 1, 0, 1),(-1, 0, 0, 1),(-1, -1, 0, 1),
                (0, 1, 1, 1),(0, -1, 1, 1),(1, 1, 1, 1),(1, -1, 1, 1),(1, 0, 1, 1),(-1, 1, 1, 1),(-1, 0, 1, 1),(-1, -1, 1, 1), (0, 0, 1, 1),
                (0, 1, -1, 1),(0, -1, -1, 1),(1, 1, -1, 1),(1, -1, -1, 1),(1, 0, -1, 1),(-1, 1, -1, 1),(-1, 0, -1, 1),(-1, -1, -1, 1), (0, 0, -1, 1),
                (0, 1, 0, -1),(0, -1, 0, -1),(1, 1, 0, -1),(1, -1, 0, -1),(1, 0, 0, -1),(-1, 1, 0, -1),(-1, 0, 0, -1),(-1, -1, 0, -1),
                (0, 1, 1, -1),(0, -1, 1, -1),(1, 1, 1, -1),(1, -1, 1, -1),(1, 0, 1, -1),(-1, 1, 1, -1),(-1, 0, 1, -1),(-1, -1, 1, -1), (0, 0, 1, -1),
                (0, 1, -1, -1),(0, -1, -1, -1),(1, 1, -1, -1),(1, -1, -1, -1),(1, 0, -1, -1),(-1, 1, -1, -1),(-1, 0, -1, -1),(-1, -1, -1, -1), (0, 0, -1, -1),
                (0, 0, 0, 1), (0, 0, 0, -1)
            ].iter() {
                neighbours.entry(p.update(*dx, *dy, *dz, *dw)).and_modify(|c| *c += 1).or_insert(1);
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
    pub fn part2_answer() {
        assert_eq!(pocket_str(default_input()), 2280)
    }
}