use grid::Grid;
use std::collections::HashMap;

pub mod part1;
pub mod part2;

pub fn run() {
    part1::run();
    part2::run();
}

pub fn default_input() -> &'static str {
    include_str!("input")
}

pub fn parse_input(input : &str) -> HashMap<i64, Picture>{
    input.split("\n\n").map(|t| {
        let mut lines = t.lines();
        let id_line = lines.next().unwrap();
        let id: i64 =  (&id_line[5..9]).parse().unwrap();
        let init_line : Vec<_> = lines.next().unwrap().chars().collect();
        let len = init_line.len();
        let mut grid = Grid::from_vec(init_line, len);
        for line in lines {
            grid.push_row(line.chars().collect())
        }
        (id, Picture {id, pixels: grid})
    }).collect()
}

pub fn compute_borders(pictures: &HashMap<i64, Picture>, picture_size : usize) -> HashMap<i64, Vec<Vec<char>>> {
    let mut border_map = HashMap::new();
    for picture in pictures.values() {
        let mut borders : Vec<Vec<char>> = Vec::new();
        borders.push(picture.pixels.iter_row(0).map(|c| *c).collect());
        borders.push(picture.pixels.iter_row(picture_size - 1).map(|c| *c).collect());
        let mut border = Vec::new();
        for c in picture.pixels.iter_col(0) {
            border.push(*c);
        }
        borders.push(border);
        let mut border = Vec::new();
        for c in picture.pixels.iter_col(picture_size - 1) {
            border.push(*c);
        }
        borders.push(border);
        border_map.insert(picture.id, borders);
    }
    border_map
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Picture {
    pub id : i64,
    pub pixels : Grid<char>
}