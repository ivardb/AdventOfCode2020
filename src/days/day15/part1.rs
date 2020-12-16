use crate::days::day15::{parse_input, default_input};
use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub fn run() {
    println!("Part 1: {}", memory_str(default_input(), 2020).unwrap());
    println!("Part 2: {}", memory_str(default_input(), 30000000).unwrap());
}

pub fn memory_str(input : &str, answer : i32) -> Result<i32, ()> {
    memory(parse_input(input), answer)
}

pub fn memory(input : Vec<i32>, answer : i32) -> Result<i32, ()> {
    let mut turn = 1;
    let mut last = input[0];
    let mut history = HashMap::new();
    for n in input.iter().skip(1) {
        history.insert(last, turn);
        turn += 1;
        last = *n;
    }
    while turn < answer {
        let val = &history.entry(last);
        let old_last = last;
        match val {
            Entry::Occupied(entry) => {
                let prev_turn = *entry.get();
                last = turn - prev_turn;
                turn += 1;
            }
            Entry::Vacant(_) => {
                last = 0;
                turn += 1;
            }
        }
        history.insert(old_last, turn - 1);
    }
    Ok(last)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn example() {
        assert_eq!(memory_str("0,3,6", 2020).unwrap(), 436)
    }

    #[test]
    pub fn part1_answer() {
        assert_eq!(memory_str(default_input(), 2020).unwrap(), 639)
    }

    #[ignore]
    #[test]
    pub fn part2_answer() {
        assert_eq!(memory_str(default_input(), 30000000).unwrap(), 266)
    }

}