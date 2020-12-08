use crate::days::day8::{Command, parse_input, default_input};
use std::collections::HashSet;

pub fn run() {
    println!("{}", infinite_loop_str(default_input()).unwrap())
}

pub fn infinite_loop_str(input : &str) -> Result<i32, ()> {
    infinite_loop(parse_input(input))
}

pub fn infinite_loop(code : Vec<Command>) -> Result<i32, ()> {
    let mut acc = 0;
    let mut pointer= 0;
    let mut executed : HashSet<usize> = HashSet::new();
    loop {
        if executed.contains(&pointer) {
            break
        }
        executed.insert(pointer);
        let command = &code[pointer as usize];
        match &*command.opcode {
            "nop" => {},
            "acc" => acc += command.arg0,
            "jmp" => {
                pointer = if command.arg0 < 0 {
                    pointer - (command.arg0 * -1) as usize
                } else {
                    pointer + command.arg0 as usize
                };
                continue
            }
            _ => {}
        }
        pointer += 1;
    }
    Ok(acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(5, infinite_loop(parse_input("nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6")).unwrap())
    }

    #[test]
    fn part1_answer() {
        assert_eq!(1337, infinite_loop_str(default_input()).unwrap())
    }
}