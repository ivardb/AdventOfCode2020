use crate::days::day14::{MemoryInstruction, parse_input, default_input};
use std::collections::HashMap;
use crate::days::day14::MemoryInstruction::{Memory, Mask};
use itertools::Itertools;

pub fn run() {
    println!("{}", mask_str(default_input()).unwrap())
}

pub fn mask_str(input : &str) -> Result<i64, ()> {
    mask(parse_input(input))
}

pub fn mask(instructions: Vec<MemoryInstruction>) -> Result<i64, ()> {
    let zero_start = i64::from_str_radix("111111111111111111111111111111111111", 2).unwrap();
    let one_start = i64::from_str_radix("000000000000000000000000000000000000", 2).unwrap();
    let mut x_list = Vec::new();
    let mut zero_mask = zero_start;
    let mut one_mask = one_start;
    let mut memory : HashMap<i64, i64> = HashMap::new();
    for instruction in &instructions {
        match instruction {
            Mask(m) => {
                zero_mask = zero_start;
                one_mask = one_start;
                x_list = Vec::new();
                let m = m.as_bytes();
                for i in 0..m.len() {
                    match m[m.len() -1 -i] as char {
                        '1' => {one_mask = one_mask | 1 << i},
                        'X' => {
                            zero_mask = zero_mask & (zero_start - (1 << i));
                            x_list.push(i);
                        },
                        _ => {}
                    }
                }
            }
            Memory(l, n) => {
                let location = (*l | one_mask) & zero_mask;
                for set in subsets(&x_list) {
                    let mut final_location = location;
                    for i in set {
                        final_location += 1 << i;
                    }
                    memory.insert(final_location, *n);
                }
            }
        }
    }
    Ok(memory.values().sum())
}

fn subsets<T: Clone>(items: &Vec<T>) -> Vec<Vec<T>> {
    (0..=items.len())
        .map(|count| items.clone().into_iter().combinations(count))
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn part2_answer() {
        assert_eq!(mask_str(default_input()).unwrap(), 2881082759597)
    }

    #[test]
    pub fn example() {
        assert_eq!(mask_str("mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1").unwrap(), 208)
    }
}