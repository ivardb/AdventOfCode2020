use crate::days::day20::{Picture, parse_input, default_input};
use std::collections::HashMap;
use grid::Grid;
use itertools::Itertools;

pub fn run() {
    println!("{}", picture_str(default_input()).unwrap())
}

pub fn picture_str(input : &str) -> Result<i64, ()> {
    picture(parse_input(input))
}

pub fn picture(pictures : Vec<Picture>) -> Result<i64, ()> {
    let picture_width = pictures[0].pixels.cols();
    let picture_height = pictures[0].pixels.rows();
    let dimension = (pictures.len() as f64).sqrt() as usize;
    let mut border_map = HashMap::new();
    for picture in &pictures {
        let mut borders : Vec<Vec<char>> = Vec::new();
        borders.push(picture.pixels.iter_row(0).map(|c| *c).collect());
        borders.push(picture.pixels.iter_row(picture_height - 1).map(|c| *c).collect());
        let mut border = Vec::new();
        for c in picture.pixels.iter_col(0) {
            border.push(*c);
        }
        borders.push(border);
        let mut border = Vec::new();
        for c in picture.pixels.iter_col(picture_width - 1) {
            border.push(*c);
        }
        borders.push(border);
        border_map.insert(picture.id, borders);
    }
    let mut matches : HashMap<BorderId, Vec<Match>> = HashMap::new();
    let mut reversed : HashMap<BorderId, Vec<char>> = HashMap::new();
    for p1 in &pictures {
        for (i, b1) in border_map.get(&p1.id).unwrap().iter().enumerate() {
            let border_id = BorderId::from_ids(p1.id, i);
            let mut reverse = b1.clone();
            reverse.reverse();
            reversed.insert(border_id, reverse);
            matches.insert(border_id, Vec::new());

            for p2 in &pictures {
                if p1.id == p2.id {
                    continue
                }
                for (j, b2) in border_map.get(&p2.id).unwrap().iter().enumerate() {
                    let border_id2 = BorderId::from_ids(p2.id, j);
                    if b2 == b1 {
                        let matched = Match {
                            matched: border_id2,
                            reversed: false
                        };
                        matches.entry(border_id)
                            .and_modify(|v| v.push(matched));
                    }
                    if b2 == reversed.get(&border_id).unwrap() {
                        let matched = Match {
                            matched: border_id2,
                            reversed: true
                        };
                        matches.entry(border_id)
                            .and_modify(|v| v.push(matched));
                    }
                }
            }
        }
    }
    let edges : Vec<_> = matches.iter().filter(|(_id, matches)| matches.len() == 0).map(|(id, _matches)| *id).collect();
    let mut count = HashMap::new();
    for id in &edges {
        *count.entry(id.id).or_insert(0) += 1;
    }
    let corners : Vec<_> = count.iter().filter(|(_k,v)| **v == 2).map(|(k, _v)| *k).collect();


    let start_corner = 1951;
    let corner_edges = edges.iter().filter(|b| b.id == start_corner).collect::<Vec<&BorderId>>();
    let start_edge = corner_edges[0];
    let top_edge = corner_edges[1];
    let mut locations : Grid<Location> = Grid::new(dimension, dimension);
    let orientation = get_orientation(top_edge.side, start_edge.side);
    locations[0][0] = Location {id: start_corner, orientation: orientation.clone()};
    let mut curr_row_edge = start_edge.clone();

    let mut curr_reversed = false;
    if orientation.contains(&Orientation::RotateLeft) {
        curr_reversed = !curr_reversed;
    }
    if orientation.contains(&Orientation::FlipVertical) {
        curr_reversed = !curr_reversed;
    }
    for i in 1..dimension {
        let matched = matches.get(&curr_row_edge.opposite()).unwrap()[0];
        let mut top_side = matched.matched.side.to_the_right();
        if matched.reversed ^ curr_reversed {
            top_side = top_side.opposite();
        }
        let orientation = get_orientation(top_side, matched.matched.side);
        locations[i][0] = Location {id: matched.matched.id, orientation: orientation.clone()};

        curr_reversed = false;
        if orientation.contains(&Orientation::RotateLeft) {
            curr_reversed = !curr_reversed;
        }
        if orientation.contains(&Orientation::FlipVertical) {
            curr_reversed = !curr_reversed;
        }
        curr_row_edge = matched.matched;
    }

    for i in 0..dimension {
        let start_location = &locations[i][0];
        let (top, _) = get_top_left(&start_location.orientation);
        let mut curr_edge = BorderId {id:start_location.id, side: top};
        let mut curr_reversed = false;
        if orientation.contains(&Orientation::RotateRight) {
            curr_reversed = !curr_reversed;
        }
        if orientation.contains(&Orientation::FlipHorizontal) {
            curr_reversed = !curr_reversed;
        }
        for j in 1..dimension {
            let matched = matches.get(&curr_edge.opposite()).unwrap()[0];
            let mut left_side = matched.matched.side.to_the_left();
            if matched.reversed ^ curr_reversed {
                left_side = left_side.opposite();
            }
            let orientation = get_orientation(matched.matched.side, left_side);
            locations[i][j] = Location {id: matched.matched.id, orientation: orientation.clone()};

            if orientation.contains(&Orientation::RotateRight) {
                curr_reversed = !curr_reversed;
            }
            if orientation.contains(&Orientation::FlipHorizontal) {
                curr_reversed = !curr_reversed;
            }
            curr_edge = matched.matched;
        }
    }

    println!("{:?}", locations);
    let mut picture_grid = Vec::new();
    for i in 0..dimension {
        let mut picture_row : Vec<Grid<char>> = Vec::new();
        for location in locations.iter_row(i) {
            picture_row.push(location.orientation.iter()
                .fold(pictures.iter().filter(|p| p.id == location.id).collect_vec()[0].pixels.clone(), |pic : Grid<char>, or| {
                    orientate(pic, *or)
                }))
        }
        picture_grid.push(picture_row)
    }
    for v in &picture_grid[0] {
        for i in 0..v.cols() {
            print!("{}", v[0][i])
        }
    }
    Err(())
}

fn orientate(grid : Grid<char>, orientation : Orientation) -> Grid<char> {
    let mut res: Grid<char> = Grid::new(grid.cols(), grid.cols());
    for i in 0..grid.rows() {
        for j in 0..grid.cols() {
            res[i][j] = match orientation {
                Orientation::FlipVertical => {grid[i][grid.cols() -1 - j]}
                Orientation::FlipHorizontal => {grid[grid.rows() -1 - i][j]}
                Orientation::RotateLeft => {grid[j][grid.cols() -1 - i]}
                Orientation::RotateRight => {grid[grid.rows() - 1 - j][i]}
            }
        }
    }
    grid
}

fn get_top_left(orientations: &Vec<Orientation>) -> (Side, Side) {
    let mut top = Side::Top;
    let mut left = Side::Left;
    for orientation in orientations {
        match orientation {
            Orientation::FlipVertical => {top = top.opposite()}
            Orientation::FlipHorizontal => {left = left.opposite()}
            Orientation::RotateLeft => {
                top = top.to_the_right();
                left = left.to_the_right();
            }
            Orientation::RotateRight => {
                top = top.to_the_left();
                left = left.to_the_left();
            }
        }
    }
    (top, left)
}

fn get_orientation(top : Side, left : Side) -> Vec<Orientation> {
    let mut orientations = Vec::new();
    match top {
        Side::Top => {}
        Side::Bottom => {
            orientations.push(Orientation::FlipVertical);
            match left {
                Side::Right => orientations.push(Orientation::FlipHorizontal),
                _ => {}
            }
        }
        Side::Left => {
            orientations.push(Orientation::RotateRight);
            match top {
                Side::Top => orientations.push(Orientation::FlipHorizontal),
                _ => {}
            }
        }
        Side::Right => {
            orientations.push(Orientation::RotateLeft);
            match top {
                Side::Bottom => orientations.push(Orientation::FlipHorizontal),
                _ => {}
            }
        }
    }
    orientations
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Location {
    id: i64,
    orientation: Vec<Orientation>
}

impl Default for Location {
    fn default() -> Self {
        Location {
            id: -1,
            orientation: Vec::new()
        }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
enum Orientation {
    FlipVertical,
    FlipHorizontal,
    RotateLeft,
    RotateRight
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Match {
    matched : BorderId,
    reversed: bool
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct BorderId {
    id: i64,
    side: Side,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
enum Side {
    Top,
    Bottom,
    Left,
    Right
}

impl BorderId {
    pub fn from_ids(id : i64, side: usize) -> Self {
        BorderId {
            id,
            side: Side::from_id(side)
        }
    }

    pub fn opposite(&self) -> Self {
        BorderId {
            id: self.id,
            side: self.side.opposite()
        }
    }
}

impl Side {
    pub fn from_id(id:usize) -> Self {
        match id {
            0 => Side::Top,
            1 => Side::Bottom,
            2 => Side::Left,
            _ => Side::Right,
        }
    }

    pub fn to_index(&self) -> usize {
        match self {
            Side::Top => 0,
            Side::Bottom => 1,
            Side::Left => 2,
            Side::Right => 3,
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            Side::Top => Side::Bottom,
            Side::Bottom => Side::Top,
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }

    pub fn to_the_right(&self) -> Self {
        match self {
            Side::Top => Side::Right,
            Side::Bottom => Side::Left,
            Side::Left => Side::Top,
            Side::Right => Side::Bottom,
        }
    }

    pub fn to_the_left(&self) -> Self {
        match self {
            Side::Top => Side::Left,
            Side::Bottom => Side::Right,
            Side::Left => Side::Bottom,
            Side::Right => Side::Top,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn example() {
        assert_eq!(20899048083289, picture_str(include_str!("example1")).unwrap())
    }
}