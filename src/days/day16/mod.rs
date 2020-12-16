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

pub fn parse_input(input : &str) -> TicketInput {
    let split : Vec<_> = input.split("\n\n").collect();
    let rules : HashMap<_,_> = split[0].lines().map(|l| {
        let kv : Vec<_> = l.split(": ").collect();
        let ranges : Vec<_> = kv[1].split(" or ").map(|v| {
            let minmax : Vec<_> = v.split("-").collect();
            Range {lower: minmax[0].parse().unwrap(), upper: minmax[1].parse().unwrap()}
        }).collect();
        (kv[0].trim().to_string(), ranges)
    }).collect();

    let ticket : Vec<i64> = split[1].lines().skip(1).next().unwrap().split(",").map(|v| v.parse().unwrap()).collect();

    let other_tickets : Vec<Vec<i64>> = split[2].lines().skip(1).map(|l| {
        l.split(",").map(|v| v.parse().unwrap()).collect()
    }).collect();

    TicketInput {rules, ticket, other_tickets}
}

pub struct TicketInput {
    pub rules: HashMap<String, Vec<Range>>,
    pub ticket: Vec<i64>,
    pub other_tickets: Vec<Vec<i64>>
}

pub struct Range {
    pub lower: i64,
    pub upper: i64,
}
