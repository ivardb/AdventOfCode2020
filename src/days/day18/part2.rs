use crate::days::day18::{default_input, Arithmetic, parse_input, evaluate_stack};

pub fn run() {
    println!("{}", evaluate_str(default_input()).unwrap());
}

pub fn evaluate_str(input : &str) -> Result<i64, ()> {
    evaluate(parse_input(input))
}

pub fn evaluate(input : Vec<Vec<Arithmetic>>) -> Result<i64, ()> {
    Ok(input.iter().map(|line| {
        let mut stack = Vec::new();
        for i in 0..line.len() {
            let ar = line[line.len()-1-i];
            match ar {
                Arithmetic::Mul => {
                    let mut total = 0;
                    let mut popped = stack.pop();
                    while popped.is_some() {
                        let ar = popped.unwrap();
                        match ar {
                            Arithmetic::Mul => {
                                stack.push(ar);
                                break;
                            }
                            Arithmetic::Add => {},
                            Arithmetic::Num(n) => total += n,
                            Arithmetic::Close => {
                                stack.push(ar);
                                break;
                            }
                            _ => break,
                        }
                        popped = stack.pop();
                    }
                    stack.push(Arithmetic::Num(total));
                    stack.push(ar);
                }
                Arithmetic::Open => {
                    evaluate_stack(&mut stack);
                }
                _ => stack.push(ar)
            }
        }
        evaluate_stack(&mut stack);
        if let Arithmetic::Num(total) = stack.pop().unwrap() {
            total
        } else {
            0
        }
    }).sum())
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn part1_answer() {
        assert_eq!(109418509151782, evaluate_str(default_input()).unwrap())
    }
}
