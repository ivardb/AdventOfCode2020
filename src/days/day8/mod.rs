use std::str::FromStr;

pub mod part1;
pub mod part2;

pub fn run() {
    part1::run();
    part2::run();
}

pub fn default_input() -> &'static str {
    include_str!("input")
}

pub fn parse_input(input : &str) -> Vec<Command> {
    input.lines().map(|line| {
        let split : Vec<_> = line.split(" ").collect();
        Command {opcode: split[0].trim().to_owned(), arg0: split[1].trim().parse().unwrap()}
    }).collect()
}

#[derive(Debug)]
pub struct Command {
    pub opcode: String,
    pub arg0: i32,
}

pub enum Instruction {
    Nop(i64),
    Jump(i64),
    Acc(i64),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(" ").collect::<Vec<_>>().as_slice() {
            ["nop", num] => Ok(Instruction::Nop(num.parse().unwrap())),
            ["jmp", num] => Ok(Instruction::Jump(num.parse().unwrap())),
            ["acc", num] => Ok(Instruction::Acc(num.parse().unwrap())),
            _ => Err(())
        }
    }
}