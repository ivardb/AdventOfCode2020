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
                        if get_adjacent(&grid, row, col).contains(&'#') {
                            new_grid[row][col] = 'L'
                        } else {
                            new_grid[row][col] = '#';
                            updated = true
                        }
                    },
                    '#' => {
                        if get_adjacent(&grid, row, col).iter().filter(|c| **c == '#').count() >= 4 {
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

fn get_adjacent(grid : &Grid<char>, row: usize, col : usize) -> Vec<char> {
    let mut res : Vec<char> = Vec::new();
    for (dx, dy) in [(1, 0), (1, -1), (1, 1), (0, 1), (0, -1), (-1, -1), (-1, 0), (-1, 1)].iter() {
        let new_row = (row as i32 + *dx) as usize;
        let new_col = (col as i32 + *dy) as usize;
        let val = grid.get(new_row, new_col);
        match val {
            Some(c) => res.push(*c),
            None => continue
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn part1_answer() {
        assert_eq!(2448, seats_str(default_input()).unwrap())
    }
}