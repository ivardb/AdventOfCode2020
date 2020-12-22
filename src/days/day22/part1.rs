use std::collections::VecDeque;
use crate::days::day22::{parse_input, default_input};

pub fn run() {
    println!("{}", combat_str(default_input()).unwrap())
}

pub fn combat_str(input : &str) -> Result<i64, ()> {
    let (p1, p2) = parse_input(input);
    combat(p1, p2)
}

pub fn combat(mut p1: VecDeque<i64>, mut p2 : VecDeque<i64>) -> Result<i64, ()> {
    while !(p1.is_empty() | p2.is_empty()) {
        let card1 = p1.pop_front().unwrap();
        let card2 = p2.pop_front().unwrap();
        if card1 > card2 {
            p1.push_back(card1);
            p1.push_back(card2);
        } else {
            p2.push_back(card2);
            p2.push_back(card1);
        }
    }
    let final_stack;
    if p1.is_empty() {
        final_stack = p2;
    } else {
        final_stack = p1;
    }
    Ok(final_stack.iter().rev().enumerate().map(|(i, v)| v * (i as i64 +1)).sum())
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn part1_answer() {
        assert_eq!(combat_str(default_input()).unwrap(), 35202)
    }
}