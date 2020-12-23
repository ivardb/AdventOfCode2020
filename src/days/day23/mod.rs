use std::collections::LinkedList;

pub mod part1;

pub fn run() {
    part1::run();
}

pub fn default_input() -> &'static str {
    "364289715"
}

pub fn parse_input(input : &str) -> Vec<i64> {
    input.chars().map(|c| c.to_digit(10).unwrap() as i64).collect()
}

pub fn parse_linked(input : &str) -> LinkedList<i64> {
    input.chars().map(|c| c.to_digit(10).unwrap() as i64).collect()
}