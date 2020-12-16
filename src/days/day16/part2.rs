use crate::days::day16::{parse_input, TicketInput};
use crate::days::day16::default_input;
use std::collections::HashMap;

pub fn run() {
    println!("{}", tickets_str(default_input()).unwrap());
}

pub fn tickets_str(input: &str) -> Result<i64, ()> {
    tickets(parse_input(input))
}

pub fn tickets(input: TicketInput) -> Result<i64, ()> {
    let valid_tickets : Vec<_> = input.other_tickets.iter().filter(|ticket| {
        ticket.iter().all(|n| {
            input.rules.values().any(|rule| rule.iter().any(|r| (*n <= r.upper) & (*n >= r.lower)))
        })
    }).collect();

    let mut options = HashMap::new();
    for (rule, ranges) in &input.rules {
        let mut valid_options = Vec::new();
        for i in 0..valid_tickets[0].len() {
            if valid_tickets.iter().all(|ticket| ranges.iter().any(|r| (ticket[i] <= r.upper) & (ticket[i] >= r.lower))) {
                valid_options.push(i);
            }
        }
        options.insert(rule, valid_options);
    }

    let mut mappings = HashMap::new();
    while mappings.len() < input.rules.len() {
        let mut match_rule = None;
        let mut match_value = None;
        for rule in options.keys() {
            let values = options.get(rule).unwrap();
            if values.len() == 1 {
                match_rule = Some(rule.clone());
                match_value = Some(values[0].clone());
                break;
            }
        }
        options.remove(match_rule.unwrap());
        options = options.iter()
            .map(|(k, v)| {
                (*k, v.iter().cloned().filter(|n| *n != match_value.unwrap()).collect())
            })
            .collect();
        mappings.insert(match_rule.unwrap(), match_value.unwrap());
    }
    Ok(mappings.keys()
        .filter(|k| (***k).starts_with("departure"))
        .map(|k| *mappings.get(k).unwrap())
        .map(|i| input.ticket[i])
        .product())
}

#[cfg(tests)]
pub mod tests {
    use super::*;

    #[test]
    pub fn part2_answer() {
        assert_eq!(tickets_str(default_input()), 51240700105297)
    }
}