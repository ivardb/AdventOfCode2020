use crate::days::day5::default_input;

pub fn run() {
    println!("{}", boarding_pass_str(default_input()).unwrap());
}

pub fn boarding_pass_str(input : &str) -> Result<i64, ()> {
    Ok(input.lines().map(|l| {
        let row_indicators = &l[..7];
        let col_indicators = &l[7..];
        let row = parse_row(row_indicators);
        let col = parse_col(col_indicators);
        row * 8 + col
    }).max().unwrap())
}

fn parse_row(rows : &str) -> i64 {
    let mut min = 0;
    let mut max = 127;
    for c in rows.chars() {
        if c == 'F' {
            max = min + (max - min) / 2
        } else {
            min = 1 + min + (max - min) / 2
        }
    }
    min
}

fn parse_col(cols : &str) -> i64 {
    let mut min = 0;
    let mut max = 7;
    for c in cols.chars() {
        if c == 'L' {
            max = min + (max - min) / 2
        } else {
            min = 1 + min + (max - min) / 2
        }
    }
    min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_answer() {
        assert_eq!(828, boarding_pass_str(default_input()).unwrap())
    }
}