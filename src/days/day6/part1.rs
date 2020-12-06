use std::collections::{HashSet, BTreeSet};
use crate::days::day6::{default_input, parse_input};

pub fn run() {
    println!("{}", customs_groups_str(default_input()).unwrap())
}

pub fn customs_groups_str(input : &str) -> Result<usize, ()> {
    customs_groups(parse_input(input))
}

pub fn customs_groups(groups : Vec<Vec<&str>>) -> Result<usize, ()> {
    let mut count = 0;
    for group in groups {
        let mut set = BTreeSet::new();
        for p in group {
            for c in p.chars() {
                set.insert(c);
            }
        }
        count += set.len();
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_answer() {
        assert_eq!(6612, customs_groups_str(default_input()).unwrap())
    }
}