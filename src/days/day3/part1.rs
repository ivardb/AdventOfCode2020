use crate::days::day3::{default_input, parse_input};
use grid;
use grid::Grid;

pub fn run() {
    let input = default_input();
    println!("{}", route_str(input).unwrap());
}

pub fn route_str(input : &str) -> Result<i32, ()> {
    route(parse_input(input).unwrap())
}

pub fn route(grid : Grid<char>) -> Result<i32, ()>{
    let width = grid.cols();
    let height = grid.rows();
    let mut x = 0;
    let mut y = 0;
    let mut count = 0;
    while y < height - 1 {
        y += 1;
        x = (x + 3) % width;
        if grid.get(y, x).unwrap() == &'#' {
            count += 1;
        }
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_answer() {
        assert_eq!(223, route_str(default_input()).unwrap())
    }
}
