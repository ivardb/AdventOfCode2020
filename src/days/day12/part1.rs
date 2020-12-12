use crate::days::day12::{Movement, parse_input, default_input};
use direction::{Direction, Coord};
use std::ops::{Add, Mul};

pub fn run() {
    println!("{}", ship_str(default_input()).unwrap())
}

pub fn ship_str(input : &str) -> Result<u32, ()> {
    ship(parse_input(input))
}

pub fn ship(input: Vec<Movement>) -> Result<u32, ()> {
    let mut dir = Direction::East;
    let mut pos = Coord{x: 0, y: 0};
    for movement in input {
        match movement {
            Movement::North(x) => {
                pos = pos.add(Direction::North.coord().mul(x));
            }
            Movement::South(x) => {
                pos = pos.add(Direction::South.coord().mul(x));
            }
            Movement::West(x) => {
                pos = pos.add(Direction::West.coord().mul(x));
            }
            Movement::East(x) => {
                pos = pos.add(Direction::East.coord().mul(x));
            }
            Movement::Left(r) => {
                match r {
                    90 => dir = dir.left90(),
                    180 => dir = dir.opposite(),
                    270 => dir = dir.right90(),
                    _ => {}
                }
            }
            Movement::Right(r) => {
                match r {
                    90 => dir = dir.right90(),
                    180 => dir = dir.opposite(),
                    270 => dir = dir.left90(),
                    _ => {}
                }
            }
            Movement::Forward(x) => {
                pos = pos.add(dir.coord().mul(x));
            }
        }
    }
    Ok(pos.manhattan_magnitude())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn part1_answer() {
        assert_eq!(1956, ship_str(default_input()).unwrap())
    }
}

