use grid::Grid;
use crate::days::day11::{parse_input, default_input};

pub fn run() {
    println!("{}", seats_str(default_input()).unwrap());
}

pub fn seats_str(input : &str) -> Result<usize, ()> {
    seats(parse_input(input))
}

pub fn seats(mut grid : Grid<char>) -> Result<usize, ()> {
    let mut updated = true;
    while updated {
        let mut new_grid = Grid::new(grid.rows(), grid.cols());
        updated = false;
        for row in 0..grid.rows() {
            for col in 0..grid.cols() {
                match grid.get(row, col).unwrap() {
                    'L' => {
                        if get_adjacent_seats(&grid, row, col) == 0 {
                            new_grid[row][col] = '#';
                            updated = true
                        } else {
                            new_grid[row][col] = 'L'
                        }
                    },
                    '#' => {
                        if get_adjacent_seats(&grid, row, col) >= 5 {
                            new_grid[row][col] = 'L';
                            updated = true
                        } else {
                            new_grid[row][col] = '#'
                        }
                    },
                    _ => new_grid[row][col] = '.'
                }
            }
        }
        grid = new_grid;
    }
    Ok(grid.iter().filter(|c| **c == '#').count())
}

fn get_adjacent_seats(grid : &Grid<char>, row: usize, col : usize) -> i32 {
    let mut count = 0;
    for (dx, dy) in [(1, 0), (1, -1), (1, 1), (0, 1), (0, -1), (-1, -1), (-1, 0), (-1, 1)].iter() {
        if find_seat(grid, row, col, *dx, *dy) {
            count += 1
        }
    }
    count
}

fn find_seat(grid : &Grid<char>, row: usize, col : usize, dx: i32, dy: i32) -> bool {
    let mut new_row = (row as i32 + dx) as usize;
    let mut new_col = (col as i32 + dy) as usize;
    let mut val = grid.get(new_row, new_col);
    while val.is_some() {
        match val.unwrap() {
            '#' => return true,
            'L' => return false,
            _ => {}
        }
        new_row = (new_row as i32 + dx) as usize;
        new_col = (new_col as i32 + dy) as usize;
        val = grid.get(new_row, new_col);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn example1() {
        assert_eq!(26, seats_str("L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL").unwrap())
    }

    #[test]
    pub fn part2_answer() {
        assert_eq!(2234, seats_str(default_input()).unwrap())
    }
}