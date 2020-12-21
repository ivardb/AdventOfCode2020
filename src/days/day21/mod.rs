use itertools::Itertools;
use std::collections::HashSet;

pub mod part1;
pub mod part2;

pub fn run() {
    part1::run();
    part2::run();
}

pub fn default_input() -> &'static str {
    include_str!("input")
}

pub fn parse_input(input : &str) -> Vec<FoodLine> {
    input.lines().map(|l| {
        let split = l.split("(contains").collect_vec();
        let ingredients = split[0].trim().split(" ").map(|s| s.to_string()).collect();
        let allergens = (&(split[1].trim())[..split[1].trim().len() -1]).split(",").map(|s| s.trim().to_string()).collect();
        FoodLine {ingredients, allergens}
    }).collect_vec()
}

pub struct FoodLine {
    pub ingredients: HashSet<String>,
    pub allergens: HashSet<String>,
}