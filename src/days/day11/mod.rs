use grid::Grid;

pub mod part1;
pub mod part2;

pub fn run() {
    part1::run();
    part2::run();
}

pub fn default_input() -> &'static str{
    include_str!("input")
}

pub fn parse_input(input : &str) -> Grid<char> {
    let init :Vec<_> = input.lines().next().unwrap().chars().collect();
    let len = &init.len();
    let mut grid = Grid::from_vec(init, *len);
    for l in input.lines().skip(1) {
        grid.push_row(l.chars().collect())
    }
    grid
}