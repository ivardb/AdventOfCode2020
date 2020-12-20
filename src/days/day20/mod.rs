use grid::Grid;

pub mod part1;
pub mod part2;

pub fn run() {
    part1::run();
    part2::run();
}

pub fn default_input() -> &'static str {
    include_str!("input")
}

pub fn parse_input(input : &str) -> Vec<Picture>{
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
        Picture {id, pixels: grid}
    }).collect()
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Picture {
    pub id : i64,
    pub pixels : Grid<char>
}