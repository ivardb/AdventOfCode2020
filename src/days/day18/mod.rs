use crate::days::day18::Arithmetic::*;

pub mod part1;
pub mod part2;

pub fn run() {
    part1::run();
    part2::run();
}

pub fn default_input() -> &'static str {
    include_str!("input")
}

pub fn parse_input(input : &str) -> Vec<Vec<Arithmetic>> {
    input.lines().map(|l| {
        let mut res = Vec::new();
        for c in l.chars() {
            match c {
                '+'=> res.push(Add),
                '*' => res.push(Mul),
                '(' => res.push(Open),
                ')' => res.push(Close),
                n if n.is_numeric() => res.push(Num(n.to_digit(10).unwrap() as i64)),
                _ => {}
            }
        }
        res
    }).collect()
}

pub(crate) fn evaluate_stack(stack : &mut Vec<Arithmetic>) {
    let mut total = 0;
    let mut op = Arithmetic::Add;
    let mut popped = stack.pop();
    while popped.is_some() {
        let ar = popped.unwrap();
        match ar {
            Arithmetic::Add | Arithmetic::Mul => op = ar,
            Arithmetic::Num(n) => {
                if op == Arithmetic::Add {
                    total += n;
                } else {
                    total *= n;
                }
            },
            _ => break,
        }
        popped = stack.pop();
    }
    stack.push(Arithmetic::Num(total));
}


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Arithmetic {
    Add,
    Mul,
    Open,
    Close,
    Num(i64)
}