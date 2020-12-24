pub mod part1;
pub mod part2;

pub fn run() {
    part1::run();
    part2::run();
}

pub fn default_input() -> &'static str {
    include_str!("input")
}

pub fn parse_input(input : &str) -> Vec<Vec<Direction>> {
    input.lines().map(|l| {
        let mut chars = l.chars();
        let mut nxt = chars.next();
        let mut res = Vec::new();
        while nxt.is_some() {
            res.push(match nxt.unwrap() {
                'e' => Direction::East,
                'n' => {
                    match chars.next().unwrap() {
                        'e' => Direction::NorthEast,
                        _ => Direction::NorthWest,
                    }
                }
                's' => {
                    match chars.next().unwrap() {
                        'e' => Direction::SouthEast,
                        _ => Direction::SouthWest,
                    }
                }
                _ => Direction::West,

            });
            nxt = chars.next();
        }
        res
    }).collect()
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl Direction {

    pub fn to_point(&self) -> Point {
        match self {
            Direction::East => Point {x: 1, y: -1, z: 0},
            Direction::SouthEast => Point {x: 0, y: -1, z: 1},
            Direction::SouthWest => Point {x: -1, y: 0, z: 1},
            Direction::West => Point {x: -1, y: 1, z: 0},
            Direction::NorthWest => Point {x: 0, y: 1, z: -1},
            Direction::NorthEast => Point {x: 1, y: 0, z: -1},
        }
    }

    pub fn all() -> Vec<Direction> {
        vec![Direction::East, Direction::SouthWest, Direction::SouthEast, Direction::West, Direction::NorthWest, Direction::NorthEast]
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct Point {
    pub x: i64, pub y:i64, pub z:i64
}

impl Point {

    pub fn add(&self, other : Point) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}