use crate::days::day7::{parse_input, default_input, Bag};
use std::collections::{HashMap, HashSet, VecDeque};

pub fn run() {
    println!("{}", color_bags_str(default_input()).unwrap())
}

pub fn color_bags_str(input : &str) -> Result<i32, ()> {
    color_bags(parse_input(input))
}

pub fn color_bags(rules : HashMap<String, Vec<Bag>>) -> Result<i32, ()> {
    let mut colors = HashSet::new();
    colors.insert("shiny gold");
    let mut queue = VecDeque::new();
    queue.push_front("shiny gold");
    let mut count = 0;
    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        for (rule, bags) in &rules {
            if colors.contains(&**rule) {
                continue
            }
            if bags.iter().any(|b| b.name == current) {
                count += 1;
                colors.insert(rule);
                queue.push_back(rule);
            }
        }
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_answer() {
        assert_eq!(335, color_bags_str(default_input()).unwrap())
    }
}