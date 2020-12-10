use crate::days::day10::{parse_input, default_input};

pub fn run() {
    println!("{}", adapters_str(default_input()).unwrap())
}

pub fn adapters_str(input : &str) -> Result<i64, ()> {
    adapters(parse_input(input))
}

pub fn adapters(mut adapters: Vec<i64>) -> Result<i64, ()> {
    adapters.sort_unstable();
    let mut options = vec![-1; adapters.len()];
    options[adapters.len() - 1] = 1;
    let mut i = options.len() - 2;
    loop {
        let mut count = 0;
        let mut j = i + 1;
        while (adapters[j] - adapters[i]) <= 3 {
            count += options[j];
            j += 1;
            if j == adapters.len() {
                break
            }
        }
        options[i] = count;
        if i == 0 {
            break
        }
        i -= 1;
    }
    let mut index = 0;
    let mut answer = 0;
    while adapters[index] <= 3 {
        answer += options[index];
        index += 1;
    }
    Ok(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn example1() {
        assert_eq!(8, adapters_str("16
10
15
5
1
11
7
19
6
12
4").unwrap())
    }

    #[test]
    pub fn part2_answer() {
        assert_eq!(64793042714624, adapters_str(default_input()).unwrap())
    }
}