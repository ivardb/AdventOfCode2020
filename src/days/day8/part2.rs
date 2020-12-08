use crate::days::day8::{Command, parse_input, default_input};
use std::collections::HashSet;

pub fn run() {
    println!("{}", infinite_loop_str(default_input()).unwrap())
}

pub fn infinite_loop_str(input : &str) -> Result<i32, ()> {
    infinite_loop(parse_input(input))
}

pub fn infinite_loop(mut code : Vec<Command>) -> Result<i32, ()> {
    let mut i = 0;
    loop {
        if code[i].opcode == "nop".to_string() {
            code[i].opcode = "jmp".to_string();
            match run_code(&code) {
                Ok(i) => return Ok(i),
                Err(_) => {}
            }
            code[i].opcode = "nop".to_string();
        } else if code[i].opcode == "jmp".to_string() {
            code[i].opcode = "nop".to_string();
            match run_code(&code) {
                Ok(i) => return Ok(i),
                Err(_) => {}
            }
            code[i].opcode = "jmp".to_string();
        }
        i += 1
    }

}

fn run_code(code : &Vec<Command>) -> Result<i32, ()> {
    let mut acc = 0;
    let mut pointer= 0;
    let mut executed : HashSet<usize> = HashSet::new();
    loop {
        if executed.contains(&pointer) {
            return Err(())
        }
        if pointer == code.len() {
            return Ok(acc)
        }
        if pointer > code.len() {
            return Err(())
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(8, infinite_loop(parse_input("nop +0
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
    fn part2_answer() {
        assert_eq!(1358, infinite_loop_str(default_input()).unwrap())
    }
}