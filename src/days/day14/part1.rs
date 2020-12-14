use crate::days::day14::{MemoryInstruction, parse_input, default_input};
use std::collections::HashMap;
use crate::days::day14::MemoryInstruction::{Memory, Mask};

pub fn run() {
    println!("{}", mask_str(default_input()).unwrap())
}

pub fn mask_str(input : &str) -> Result<i64, ()> {
    mask(parse_input(input))
}

pub fn mask(instructions: Vec<MemoryInstruction>) -> Result<i64, ()> {
    let zero_start = i64::from_str_radix("111111111111111111111111111111111111", 2).unwrap();
    let one_start = i64::from_str_radix("000000000000000000000000000000000000", 2).unwrap();
    let mut zero_mask = zero_start;
    let mut one_mask = one_start;
    let mut memory : HashMap<i64, i64> = HashMap::new();
    for instruction in &instructions {
        match instruction {
            Mask(m) => {
                zero_mask = zero_start;
                one_mask = one_start;
                let m = m.as_bytes();
                for i in 0..m.len() {
                    match m[m.len() -1 -i] as char {
                        '1' => {one_mask = one_mask | 1 << i},
                        '0' => {zero_mask = zero_mask & (zero_start - (1 << i))},
                        _ => {}
                    }
                }
            }
            Memory(l, n) => {
                let val = (n | one_mask) & zero_mask;
                memory.insert(*l, val);
            }
        }
    }
    Ok(memory.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn part1_answer() {
        assert_eq!(mask_str(default_input()).unwrap(), 10452688630537)
    }

    #[test]
    pub fn example() {
        assert_eq!(mask_str("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0").unwrap(), 165)
    }
}