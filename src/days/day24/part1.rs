use crate::days::day24::{Direction, default_input, parse_input, Point};
use std::collections::HashSet;

pub fn run() {
    println!("{}", hex_str(default_input()).unwrap());
}

pub fn hex_str(input : &str) -> Result<usize, ()> {
    hex(parse_input(input))
}

pub fn hex(input : Vec<Vec<Direction>>) -> Result<usize, ()> {
    let mut black = HashSet::new();
    for directions in input {
        let mut point = Point {x: 0, y:0, z:0};
        for direction in directions {
            point = point.add(direction.to_point());
        }
        if black.contains(&point) {
            black.remove(&point);
        } else {
            black.insert(point);
        }
    }
    Ok(black.len())
}