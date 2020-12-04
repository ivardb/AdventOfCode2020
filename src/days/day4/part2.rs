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
        if passport.contains_key("cid") {
            return false;
        }
    }
    if passport.len() < 7 {
        return false;
    }
    for key in passport.keys() {
        let value = passport.get(key).unwrap();
        match key.trim() {
            "byr" => {
                match value.parse::<i32>() {
                    Ok(n) => if n < 1920 || n > 2002 {return false;}
                    Err(_) => return false
                }
            },
            "iyr" => {
                match value.parse::<i32>() {
                    Ok(n) => if n < 2010 || n > 2020 {return false;}
                    Err(_) => return false
                }
            },
            "eyr" => {
                match value.parse::<i32>() {
                    Ok(n) => if n < 2020 || n > 2030 {return false;}
                    Err(_) => return false
                }
            },
            "hgt" => {
                match value[0 .. value.len()-2].parse::<i32>() {
                    Ok(n) => {
                        if value.ends_with("cm") {
                            if n < 150 || n > 193 {
                                return false
                            }
                        } else {
                            if n < 59 || n > 76 {
                                return false
                            }
                        }
                    },
                    Err(_) => return false
                }
            },
            "hcl" => {
                if value.len() != 7 {
                    return false
                }
                if value.as_bytes()[0] as char != '#' {
                    return false
                }
                for c in value.chars().skip(1) {
                    if !c.is_alphanumeric() {
                        return false
                    }
                }
            },
            "ecl" => {
                if value.len() != 3 {
                    return false
                }
                if !vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&&**value) {
                    return false
                }
            },
            "pid" => {
                if value.len() != 9 {
                    return false
                }
                for c in value.chars() {
                    if !c.is_numeric() {
                        return false
                    }
                }
            }
            _ => continue
        }
    }
    true
}