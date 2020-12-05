use std::collections::HashMap;

pub mod part1;
pub mod part2;

pub fn default_input()  -> &'static str {
    include_str!("input")
}

pub fn run() {
    part1::run();
    part2::run();
}

pub fn parse_input(input : &str) -> Vec<HashMap<String, String>> {
    input.split("\n\n")
        .map(|p| {
            let map : HashMap<String, String> = p.split(&['\n', ' '][..])
                .map(|l| {
                    let split : Vec<_> = l.split(":").collect();
                    (String::from(split[0].trim()), String::from(split[1]))})
                .collect();
            map
        })
        .collect()
}