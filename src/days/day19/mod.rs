use std::collections::HashMap;

pub mod part1;
pub mod part2;

pub fn run() {
    part1::run();
    part2::run();
}

pub fn default_input() -> &'static str {
    include_str!("input")
}

pub fn parse_input(input : &str) -> MessageInput {
    let split : Vec<_>= input.split("\n\n").collect();
    let rules = split[0].lines().map(|l| {
        let kv : Vec<_> = l.split(":").collect();
        let values : Vec<_> = kv[1].split("|").collect();
        let vals : Vec<Vec<Value>> = values.iter().map(|v| {
            if v.contains("\"") {
                let mut vec = Vec::new();
                vec.push(Value::Term(v.trim().split("\"").skip(1).next().unwrap().parse().unwrap()));
                vec
            } else {
                let nums = v.trim().split(" ");
                nums.map(|n| Value::Num(n.trim().parse().unwrap())).collect()
            }
        }).collect();
        (kv[0].trim().parse().unwrap(), vals)
    }).collect();
    let messages = split[1].lines().map(|l| l.to_string()).collect();
    MessageInput {rules, messages}
}

pub struct MessageInput {
    pub rules : HashMap<i64, Vec<Vec<Value>>>,
    pub messages: Vec<String>
}

pub enum Value {
    Num(i64),
    Term(char),
}