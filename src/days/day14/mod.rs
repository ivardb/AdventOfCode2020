use crate::days::day14::MemoryInstruction::{Mask, Memory};

pub mod part1;
pub mod part2;

pub fn run() {
    part1::run();
    part2::run();
}

pub fn default_input() -> &'static str {
    include_str!("input")
}

pub fn parse_input(input : &str) -> Vec<MemoryInstruction>{
    input.lines().map(|l| {
        let split : Vec<_>  = l.split(" = ").collect();
        if split[0] == "mask" {
            Mask(split[1].to_string())
        } else {
            let split2 : Vec<_> = split[0].split(&['[', ']'][..]).collect();
            Memory(split2[1].parse().unwrap(), split[1].parse().unwrap())
        }
    }).collect()
}

#[derive(Debug)]
pub enum MemoryInstruction {
    Mask(String),
    Memory(i64, i64)
}