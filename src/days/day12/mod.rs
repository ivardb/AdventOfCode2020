use crate::days::day12::Movement::*;

pub mod part1;
pub mod part2;

pub fn run() {
    part1::run();
    part2::run();
}

pub fn default_input() -> &'static str {
    include_str!("input")
}

pub fn parse_input(input : &str) -> Vec<Movement> {
    input.lines().map(|l| {
        let inst = l.as_bytes()[0] as char;
        let num : i32 = l.replace(inst, "").parse().unwrap();
        match inst {
            'N' => North(num),
            'S' => South(num),
            'W' => West(num),
            'E' => East(num),
            'L' => Left(num),
            'R' => Right(num),
            _ => Forward(num)
        }
    }).collect()
}

pub enum Movement {
    North(i32),
    South(i32),
    West(i32),
    East(i32),
    Left(i32),
    Right(i32),
    Forward(i32)
}