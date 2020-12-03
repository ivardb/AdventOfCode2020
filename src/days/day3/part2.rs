use crate::days::day3::default_input;
use grid::Grid;

pub fn run() {
    let input = default_input();
    println!("{}", route_str(input).unwrap());
}

pub fn route_str(input : &str) -> Result<i64, ()>{
    let init :Vec<_> = input.lines().next().unwrap().chars().collect();
    let len = &init.len();
    let mut grid = Grid::from_vec(init, *len);
    for l in input.lines().skip(1) {
        grid.push_row(l.chars().collect())
    }
    Ok(check_route(&grid, 1, 1) *
        check_route(&grid, 3, 1) *
        check_route(&grid, 5, 1) *
        check_route(&grid, 7, 1) *
        check_route(&grid, 1, 2))
}

fn check_route(grid : &Grid<char>, dx : usize, dy : usize) -> i64 {
    let width = grid.cols();
    let height = grid.rows();
    let mut x = 0;
    let mut y = 0;
    let mut count = 0;
    while y < height - dy {
        y += dy;
        x = (x + dx) % width;
        if grid.get(y, x).unwrap() == &'#' {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_answer() {
        assert_eq!(3517401300, route_str(default_input()).unwrap())
    }
}