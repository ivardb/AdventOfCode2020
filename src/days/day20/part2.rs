use crate::days::day20::{Picture, parse_input, default_input};
use std::collections::HashMap;
use grid::Grid;
use itertools::Itertools;

pub fn run() {
    println!("{}", picture_str(default_input()).unwrap())
}

pub fn picture_str(input : &str) -> Result<usize, ()> {
    picture(parse_input(input))
}

pub fn picture(pictures : Vec<Picture>) -> Result<usize, ()> {
    let picture_map : HashMap<_,_> = pictures.iter().map(|p| (p.id, p.clone())).collect();
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
    let corners : Vec<_> = count.iter().filter(|(_k,v)| **v == 2).map(|(k, _v)| *k).sorted().collect();


    let start_corner = corners[0];


    let corner_edges = edges.iter().filter(|b| b.id == start_corner).sorted_by_key(|e| e.side.to_index()).collect::<Vec<&BorderId>>();
    let start_edge = corner_edges[1];
    let top_edge = corner_edges[0];
    let orientation = get_orientation(top_edge.side, start_edge.side);
    let rotated = orientation.iter().fold(picture_map.get(&start_corner).unwrap().pixels.clone(), |p, o| orientate(p, *o));
    let mut picture_cols: Vec<Vec<Grid<char>>> = Vec::new();
    for _i in 0..dimension {
        picture_cols.push(Vec::new());
    }
    picture_cols[0].push(rotated.clone());
    let mut last_col_id = start_edge.opposite();
    let mut last_col = rotated.iter_col(rotated.cols() - 1).map(|c| *c).collect_vec();
    let mut ids = Vec::new();
    ids.push(start_corner);
    for i in 1..dimension {
        let matched = matches.get(&last_col_id).unwrap()[0];
        let matched_grid = picture_map.get(&matched.matched.id).unwrap().pixels.clone();
        for orientation in possible_orientations() {
            let rotate = orientation.iter().fold(matched_grid.clone(), |p, o| orientate(p, *o));
            let first_col = rotate.iter_col(0).map(|c| *c).collect_vec();
            if first_col == last_col {
                last_col = rotate.iter_col(rotate.cols() -1).map(|c| *c).collect_vec();
                last_col_id = matched.matched.opposite();
                picture_cols[i].push(rotate);
                ids.push(matched.matched.id);
                break;
            }
        }
    }

    for i in 0..dimension {
        let old_pic = &picture_cols[i][0];
        let mut last_row = old_pic.iter_row(old_pic.rows() - 1).map(|c| *c).collect_vec();
        let id = ids[i];
        let side = get_side(&last_row, border_map.get(&id).unwrap()).unwrap();
        let mut last_row_id = BorderId {id, side};
        for _j in 1..dimension {
            let matched = matches.get(&last_row_id).unwrap()[0];
            let matched_grid = picture_map.get(&matched.matched.id).unwrap().pixels.clone();
            for orientation in possible_orientations() {
                let rotate = orientation.iter().fold(matched_grid.clone(), |p, o| orientate(p, *o));
                let first_row = rotate.iter_row(0).map(|c| *c).collect_vec();
                if first_row == last_row {
                    last_row = rotate.iter_row(rotate.rows() - 1).map(|c| *c).collect_vec();
                    last_row_id = matched.matched.opposite();
                    picture_cols[i].push(rotate);
                    break;
                }
            }
        }
    }
    let mut picture = Vec::new();
    for ygrid in 0..dimension {
        for j in 1..picture_height -1 {
            let mut picture_row = Vec::new();
            for i in 0..dimension {
                for k in 1..picture_width - 1 {
                    picture_row.push(picture_cols[i][ygrid][j][k])
                }
            }
            picture.push(picture_row);
        }
    }

    //Find monsters
    let mut found = false;
    for orientations in possible_orientations() {
        let mut after_move_picture = orientations.iter().fold(picture.clone(), |pic, o| {
            orientate_vec(&pic, *o)
        });
        for i in 1..after_move_picture.len() - 1 {
            for j in 0..after_move_picture.len() - 19 {
                if find_monster(&mut after_move_picture, i, j) {
                    found = true;
                }
            }
        }
        if found == true {
            return Ok(after_move_picture.iter().flatten().filter(|c| **c == '#').count())
        }
    }
    Err(())
}

fn get_side(row: &Vec<char>, borders: &Vec<Vec<char>>) -> Result<Side, ()> {
    let mut reverse = row.clone();
    reverse.reverse();
    for (i, border) in borders.iter().enumerate() {
        if row == border {
            return Ok(Side::from_id(i))
        } else if &reverse == border {
            return Ok(Side::from_id(i))
        }
    }
    Err(())
}

pub fn picture2(pictures : Vec<Picture>) -> Result<usize, ()> {
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
    let corners : Vec<_> = count.iter().filter(|(_k,v)| **v == 2).map(|(k, _v)| *k).sorted().collect();


    let start_corner = 1951;//corners[0];


    let corner_edges = edges.iter().filter(|b| b.id == start_corner).sorted_by_key(|e| e.side.to_index()).collect::<Vec<&BorderId>>();
    let start_edge = corner_edges[1];
    let top_edge = corner_edges[0];
    let mut locations : Grid<Location> = Grid::new(dimension, dimension);
    let orientation = get_orientation(top_edge.side, start_edge.side);


    locations[0][0] = Location {id: start_corner, orientation: orientation.clone()};
    let mut curr_row_edge = start_edge.clone();

    let mut curr_reversed = match (top_edge.side, start_edge.side) {
        (Side::Top, _) => false,
        (Side::Bottom, _) => true,
        (Side::Right, Side::Top) => true,
        (Side::Right, Side::Bottom) => false,
        (Side::Left, Side::Top) => false,
        (Side::Left, Side::Bottom) => true,
        (_, _) => false
    };
    for i in 1..dimension {
        let matched = matches.get(&curr_row_edge.opposite()).unwrap()[0];
        let mut top_side = match matched.matched.side {
            Side::Top => {Side::Right}
            Side::Bottom => {Side::Left}
            Side::Left => {Side::Top}
            Side::Right => {Side::Top}
        };

        curr_reversed =  matched.reversed ^ curr_reversed;
        if curr_reversed {
            top_side = top_side.opposite();
        }

        let orientation = get_orientation(top_side, matched.matched.side);
        locations[0][i] = Location {id: matched.matched.id, orientation: orientation.clone()};

        curr_row_edge = matched.matched;
    }

    for i in 0..dimension {
        let start_location = &locations[0][i];
        let (top, left) = get_top_left(&start_location.orientation);
        println!("{:?}", get_top_left(&start_location.orientation));
        let mut curr_edge = BorderId {id:start_location.id, side: top};
        let mut curr_reversed = match (top, left) {
            (_, Side::Right) => true,
            (_, Side::Left) => false,
            (Side::Left, Side::Top) => false,
            (Side::Left, Side::Bottom) => true,
            (Side::Right, Side::Top) => true,
            (Side::Right, Side::Bottom) => false,
            (_, _) => false
        };
        for j in 1..dimension {
            println!("i:{}, j: {}, location: {:?}, curr_edge: {:?}", i, j, &locations[0][i].clone(), &curr_edge);
            let matched = matches.get(&curr_edge.opposite()).unwrap()[0];

            let mut left_side = match matched.matched.side {
                Side::Top => {Side::Left}
                Side::Bottom => {Side::Left}
                Side::Left => {Side::Bottom}
                Side::Right => {Side::Top}
            };
            curr_reversed =  matched.reversed ^ curr_reversed;
            if curr_reversed {
                left_side = left_side.opposite();
            }
            let orientation = get_orientation(matched.matched.side, left_side);
            locations[j][i] = Location {id: matched.matched.id, orientation: orientation.clone()};

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

    //Construct final picture
    let mut picture = Vec::new();
    for i in 0..dimension {
        for j in 1..picture_height -1 {
            let mut picture_row = Vec::new();
            for v in &picture_grid[i] {
                for k in 1..v.cols() -1 {
                    picture_row.push(v[j][k]);
                }
            }
            picture.push(picture_row);
        }
    }

    let orientations = vec![Orientation::FlipHorizontal, Orientation::FlipVertical];
    let test_pic = orientations.iter().fold(picture.clone(), |pic, o| {
        orientate_vec(&pic, *o)
    });
    for v in &test_pic {
        for c in v {
            print!("{}", c);
        }
        println!()
    }

    //Find monsters
    let mut found = false;
    for orientations in possible_orientations() {
        let mut after_move_picture = orientations.iter().fold(picture.clone(), |pic, o| {
            orientate_vec(&pic, *o)
        });
        for i in 1..after_move_picture.len() - 1 {
            for j in 0..after_move_picture.len() - 19 {
                if find_monster(&mut after_move_picture, i, j) {
                    found = true;
                }
            }
        }
        if found == true {
            return Ok(after_move_picture.iter().flatten().filter(|c| **c == '#').count())
        }
    }
    Err(())
}

fn possible_orientations() -> Vec<Vec<Orientation>> {
    vec![
        get_orientation(Side::Top, Side::Left),
        get_orientation(Side::Top, Side::Right),
        get_orientation(Side::Bottom, Side::Left),
        get_orientation(Side::Bottom, Side::Right),
        get_orientation(Side::Left, Side::Top),
        get_orientation(Side::Left, Side::Bottom),
        get_orientation(Side::Right, Side::Top),
        get_orientation(Side::Right, Side::Bottom)
    ]
}

fn find_monster(picture : &mut Vec<Vec<char>>, x:usize, y:usize) -> bool {
    let pattern = [(0,0), (1,1), (1, 4), (0, 5), (0,6), (1,7), (1, 10), (0, 11), (0, 12), (1, 13), (1, 16), (0, 17), (0, 18), (-1, 18), (0, 19)];
    if pattern.iter().all(|(dx, dy)| {
        let newx = (x as i64 + dx) as usize;
        let newy = (y as i64 + dy) as usize;
        picture[newx][newy] == '#'
    }) {
        for (dx, dy) in &pattern {
            let newx = (x as i64 + dx) as usize;
            let newy = (y as i64 + dy) as usize;
            picture[newx][newy] = '0';
        }
        true
    } else {
        false
    }
}

fn print_grid(input : &Grid<char>) {
    for i in 0..input.rows() {
        for j in 0..input.cols() {
            print!("{}", input[i][j])
        }
        println!()
    }
}

fn orientate_vec(vector: &Vec<Vec<char>>, orientation : Orientation) -> Vec<Vec<char>> {
    let mut res = Vec::new();
    for i in 0..vector.len() {
        let mut vec_row = Vec::new();
        for j in 0..vector[i].len() {
            vec_row.push( match orientation {
                Orientation::FlipHorizontal => {vector[i][vector.len() -1 - j]}
                Orientation::FlipVertical => {vector[vector.len() -1 - i][j]}
                Orientation::RotateLeft => {vector[j][vector.len() -1 - i]}
                Orientation::RotateRight => {vector[vector.len() - 1 - j][i]}
            })
        }
        res.push(vec_row);
    }
    res
}

fn orientate(grid : Grid<char>, orientation : Orientation) -> Grid<char> {
    let mut res: Grid<char> = Grid::new(grid.cols(), grid.cols());
    for i in 0..grid.rows() {
        for j in 0..grid.cols() {
            res[i][j] = match orientation {
                Orientation::FlipHorizontal => {grid[i][grid.cols() -1 - j]}
                Orientation::FlipVertical => {grid[grid.rows() -1 - i][j]}
                Orientation::RotateLeft => {grid[j][grid.cols() -1 - i]}
                Orientation::RotateRight => {grid[grid.rows() - 1 - j][i]}
            }
        }
    }
    res
}

fn get_top_left(orientations: &Vec<Orientation>) -> (Side, Side) {
    let mut top = Side::Top;
    let mut left = Side::Left;
    for orientation in orientations {
        match orientation {
            Orientation::FlipVertical => {top = top.opposite()}
            Orientation::FlipHorizontal => {left = left.opposite()}
            Orientation::RotateRight => {
                top = top.to_the_right();
                left = left.to_the_right();
            }
            Orientation::RotateLeft => {
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
        Side::Top => {
            match left {
                Side::Right => orientations.push(Orientation::FlipHorizontal),
                _ => {}
            }
        }
        Side::Bottom => {
            orientations.push(Orientation::FlipVertical);
            match left {
                Side::Right => orientations.push(Orientation::FlipHorizontal),
                _ => {}
            }
        }
        Side::Left => {
            orientations.push(Orientation::RotateLeft);
            match left {
                Side::Top => orientations.push(Orientation::FlipHorizontal),
                _ => {}
            }
        }
        Side::Right => {
            orientations.push(Orientation::RotateRight);
            match left {
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
        assert_eq!(273, picture_str(include_str!("example1")).unwrap())
    }
}