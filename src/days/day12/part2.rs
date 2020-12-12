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
    let mut pos = Coord{x: 0, y: 0};
    let mut waypoint = Coord{x: 10, y: -1};
    for movement in input {
        match movement {
            Movement::North(x) => {
                waypoint = waypoint.add(Direction::North.coord().mul(x));
            }
            Movement::South(x) => {
                waypoint = waypoint.add(Direction::South.coord().mul(x));
            }
            Movement::West(x) => {
                waypoint = waypoint.add(Direction::West.coord().mul(x));
            }
            Movement::East(x) => {
                waypoint = waypoint.add(Direction::East.coord().mul(x));
            }
            Movement::Left(r) => {
                match r {
                    90 => waypoint = waypoint.left90(),
                    180 => waypoint = waypoint.opposite(),
                    270 => waypoint = waypoint.right90(),
                    _ => {}
                }
            }
            Movement::Right(r) => {
                match r {
                    90 => waypoint = waypoint.right90(),
                    180 => waypoint = waypoint.opposite(),
                    270 => waypoint = waypoint.left90(),
                    _ => {}
                }
            }
            Movement::Forward(x) => {
                pos = pos.add(waypoint.mul(x));
            }
        }
    }
    Ok(pos.manhattan_magnitude())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn part2_answer() {
        assert_eq!(126797, ship_str(default_input()).unwrap())
    }
}