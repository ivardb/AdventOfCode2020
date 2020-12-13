use crate::days::day13::default_input;

pub fn run() {
    println!("{}", busses_str(default_input()).unwrap())
}

pub fn busses_str(input : &str) -> Result<i32, ()> {
    let mut it = input.lines();
    let input : i32 = it.next().unwrap().parse().unwrap();
    let busses: Vec<i32> = it.next().unwrap().split(",").filter(|p| *p != "x").map(|n| n.parse().unwrap()).collect();
    let mut min = i32::max_value();
    let mut min_value = -1;
    for bus in busses {
        let m = (input/bus + 1) * bus - input;
        if m < min {
            min = m;
            min_value = bus;
        }
    }
    Ok(min * min_value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn example() {
        assert_eq!(busses_str("939
7,13,x,x,59,x,31,19").unwrap(), 295)
    }
}