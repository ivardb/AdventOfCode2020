use crate::days::day7::{Bag, parse_input, default_input};
use std::collections::HashMap;

pub fn run() {
    println!("{}", color_bags_str(default_input()).unwrap())
}

pub fn color_bags_str(input : &str) -> Result<u32, ()> {
    color_bags(parse_input(input))
}

pub fn color_bags(rules : HashMap<String, Vec<Bag>>) -> Result<u32, ()> {
    Ok(get_bag_count(&rules, "shiny gold") - 1)
}

fn get_bag_count(rules : &HashMap<String, Vec<Bag>>, name: &str) -> u32 {
    rules.get(name).unwrap().iter().map(|b| b.amount * get_bag_count(rules, &*b.name)).sum::<u32>() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_answer() {
        assert_eq!(2431, color_bags_str(default_input()).unwrap())
    }
}