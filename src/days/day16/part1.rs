use crate::days::day16::{parse_input, TicketInput};
use crate::days::day16::default_input;

pub fn run() {
    println!("{}", tickets_str(default_input()).unwrap());
}

pub fn tickets_str(input: &str) -> Result<i64, ()> {
    tickets(parse_input(input))
}

pub fn tickets(input: TicketInput) -> Result<i64, ()> {
    Ok(input.other_tickets.iter().flatten().filter(|n| {
        !input.rules.values().any(|vector| vector.iter().any(|r| (**n <= r.upper) & (**n >= r.lower)))
    }).sum())
}

#[cfg(tests)]
pub mod tests {
    use super::*;

    #[test]
    pub fn part1_answer() {
        assert_eq!(tickets_str(default_input()), 23054)
    }
}