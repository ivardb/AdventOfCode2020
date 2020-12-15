pub mod part1;

pub fn run() {
    part1::run();
}

pub fn default_input() -> &'static str {
    include_str!("input")
}

pub fn parse_input(input : &str) -> Vec<i32> {
    input.split(",").map(|n| n.trim().parse().unwrap()).collect()
}