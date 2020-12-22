use std::collections::{VecDeque, HashSet};
use crate::days::day22::{parse_input, default_input};

pub fn run() {
    println!("{}", combat_str(default_input()).unwrap())
}

pub fn combat_str(input : &str) -> Result<i64, ()> {
    let (p1, p2) = parse_input(input);
    combat(p1, p2)
}

pub fn combat(mut p1: VecDeque<i64>, mut p2 : VecDeque<i64>) -> Result<i64, ()> {
    simulate_game(&mut p1, &mut p2);
    let final_stack;
    if p1.is_empty() {
        final_stack = p2;
    } else {
        final_stack = p1;
    }
    Ok(calculate_score(&final_stack))
}

fn calculate_score(stack : &VecDeque<i64>) -> i64 {
    stack.iter().rev().enumerate().map(|(i, v)| v * (i as i64 +1)).sum()
}

fn combat_winner(mut p1: VecDeque<i64>, mut p2 : VecDeque<i64>) -> bool {
    if p1.iter().max() > p2.iter().max() {
        return true
    }
    simulate_game(&mut p1, &mut p2);
    if p1.is_empty() {
        false
    } else {
        true
    }
}

fn simulate_game(p1 : &mut VecDeque<i64>, p2 : &mut VecDeque<i64>) {
    let mut happened: HashSet<(i64, i64)> = HashSet::new();
    while !(p1.is_empty() | p2.is_empty()) {
        if !happened.insert((calculate_score(&p1), calculate_score(&p2))) {
            p2.clear();
            break;
        }
        let card1 = p1.pop_front().unwrap();
        let card2 = p2.pop_front().unwrap();
        if (p1.len() as i64 >= card1) & (p2.len() as i64 >= card2) {
            if combat_winner(p1.iter().take(card1 as usize).map(|n| *n).collect(), p2.iter().take(card2 as usize).map(|n| *n).collect()) {
                p1.push_back(card1);
                p1.push_back(card2);
            } else {
                p2.push_back(card2);
                p2.push_back(card1);
            }
        } else {
            if card1 > card2 {
                p1.push_back(card1);
                p1.push_back(card2);
            } else {
                p2.push_back(card2);
                p2.push_back(card1);
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn example() {
        assert_eq!(combat_str("Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10").unwrap(), 291)
    }

    #[test]
    pub fn part2_answer() {
        assert_eq!(combat_str(default_input()).unwrap(), 32317)
    }
}