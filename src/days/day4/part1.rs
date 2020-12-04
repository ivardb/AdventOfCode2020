use crate::days::day4::{default_input, parse_input};
use std::collections::HashMap;

pub fn run() {
    println!("{}", password_system_str(default_input()).unwrap());
}

pub fn password_system_str(input : &str) -> Result<usize, ()> {
    password_system(parse_input(input))
}

pub fn password_system(passports : Vec<HashMap<String, String>>) -> Result<usize, ()> {
    Ok(passports.iter().filter(|p| {valid_passport(p)}).count())
}

fn valid_passport(passport : &HashMap<String, String>) -> bool {
    if passport.len() == 7 {
        return !passport.contains_key("cid")
    }
    passport.len() == 8
}