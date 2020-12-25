use itertools::Itertools;

pub mod part1;

pub fn run() {
    part1::run();
}

pub fn default_input() -> &'static str {
    include_str!("input")
}

pub fn parse_input(input : &str) -> Keys {
    let lines = input.lines().collect_vec();
    let door = lines[0].parse().unwrap();
    let card = lines[1].parse().unwrap();
    Keys { door, card }
}

pub struct Keys {
    pub door: i64,
    pub card: i64,
}