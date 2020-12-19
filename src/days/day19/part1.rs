use crate::days::day19::{MessageInput, parse_input, default_input, Value};
use std::collections::HashMap;
use regex::Regex;

pub fn run() {
    println!("{}", message_str(default_input()).unwrap())
}

pub fn message_str(input : &str) -> Result<usize, ()> {
    message(parse_input(input))
}

pub fn message(input : MessageInput) -> Result<usize, ()> {
    let rule = format!("^{}$", parse_rules(&input.rules, 0));
    let regex = Regex::new(&*rule).unwrap();
    Ok(input.messages.iter().filter(|m| {
        regex.is_match(m)
    }).count())
}

fn parse_rules(rules : &HashMap<i64, Vec<Vec<Value>>>, num : i64) -> String {
    let rule = rules.get(&num).unwrap();
    match rule[0][0] {
        Value::Term(c) => return c.to_string(),
        _ => {}
    }
    if rule.len() == 1 {
        rule[0].iter().fold(String::new(), |res, n| {
            match n {
                Value::Num(x) => res + &*parse_rules(rules, *x),
                Value::Term(c) => res + &*format!("{}", c)
            }
        })
    } else {
        let mut res = String::from("(");
        for (i, rule) in rule.iter().enumerate() {
            if i != 0 {
                res = res + "|";
            }
            res = res + &*rule.iter().fold(String::new(), |res, n| {
                match n {
                    Value::Num(x) => res + &*parse_rules(rules, *x),
                    _ => res
                }
            })
        }
        res + ")"
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn example() {
        assert_eq!(message_str(default_input()).unwrap(), 156)
    }
}