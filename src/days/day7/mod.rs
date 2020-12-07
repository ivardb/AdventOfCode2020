use itertools::__std_iter::FromIterator;
use std::collections::HashMap;

pub mod part1;
pub mod part2;

pub fn default_input() -> &'static str{
    include_str!("input")
}

pub fn run() {
    part1::run();
    part2::run();
}

pub fn parse_input(input : &str) -> HashMap<String, Vec<Bag>> {
    input.split("\n").map(|line| {
        let split : Vec<_> = line.split("contain").collect();
        let mut values: Vec<_> = split[1].split(&[',', '.'][..]).collect();
        values.remove(values.len()-1);
        let bags : Vec<Bag>;
        if values[0].trim().as_bytes()[0].is_ascii_alphabetic() {
            bags = Vec::new();
        } else {
            bags = values.iter().map(|s| {
                let amount = (s.trim().as_bytes()[0] as char).to_digit(10).unwrap();
                let name = String::from_iter(s.trim().chars().skip(1)).replace("bags", "").replace("bag", "").trim().to_owned();
                Bag { amount, name }
            }).collect();
        }
        (split[0].replace("bags", "").replace("bag", "").trim().to_owned(), bags)
    }).collect()
}

pub struct Bag {
    pub amount: u32,
    pub name: String,
}