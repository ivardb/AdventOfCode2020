use crate::days::day4::default_input;
use std::collections::HashMap;

pub fn run() {
    let input = default_input();
    println!("{}", password_system_str(input).unwrap());
}

pub fn password_system_str(input : &str) -> Result<usize, ()> {
    let passports : Vec<_> = input.split("\r\n\r")
        .map(|p| {
            let map : HashMap<String, String> = p.split(&['\r', ' '][..])
                .map(|l| {
                    let split : Vec<_> = l.split(":").collect();
                    (String::from(split[0].trim()), String::from(split[1]))})
                .collect();
            map
        })
        .collect();
    Ok(passports.iter().filter(|p| {valid_passport(p)}).count())
}

fn valid_passport(passport : &HashMap<String, String>) -> bool {
    if passport.len() == 7 {
        return !passport.contains_key("cid")
    }
    passport.len() == 8
}