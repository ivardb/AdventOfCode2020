use crate::days::day20::{Picture, parse_input, default_input, compute_borders};
use std::collections::HashMap;
use grid::Grid;
use itertools::Itertools;

pub fn run() {
    println!("{}", picture_str(default_input()).unwrap())
}

pub fn picture_str(input : &str) -> Result<usize, ()> {
    picture(parse_input(input))
}

pub fn picture(pictures : HashMap<i64, Picture>) -> Result<usize, ()> {
    let picture_size = pictures.values().next().unwrap().pixels.cols();
    let dimension = (pictures.values().len() as f64).sqrt() as usize;

    let border_map = compute_borders(&pictures, picture_size);

    //Compute all matches between borders
    let mut matches : HashMap<BorderId, Vec<BorderId>> = HashMap::new();
    for p1 in pictures.values() {
        for (i, b1) in border_map.get(&p1.id).unwrap().iter().enumerate() {
            let border_id = BorderId::from_ids(p1.id, i);

            //Create reverse
            let mut reverse = b1.clone();
            reverse.reverse();

            //Ensure empty vectors to filter out edges later
            matches.insert(border_id, Vec::new());

            for p2 in pictures.values() {
                if p1.id == p2.id {
                    continue
                }
                for (j, b2) in border_map.get(&p2.id).unwrap().iter().enumerate() {
                    let border_id2 = BorderId::from_ids(p2.id, j);
                    if (b2 == b1) | (b2 == &reverse) {
                        matches.entry(border_id)
                            .and_modify(|v| v.push(border_id2));
                    }
                }
            }
        }
    }
    //Filter out edges
    let edges : Vec<_> = matches.iter().filter(|(_id, matches)| matches.len() == 0).map(|(id, _matches)| *id).collect();

    //Get count per picture id to get corners
    let mut count = HashMap::new();
    for id in &edges {
        *count.entry(id.id).or_insert(0) += 1;
    }
    let corners : Vec<_> = count.iter().filter(|(_k,v)| **v == 2).map(|(k, _v)| *k).collect();

    //Get starting corner
    let start_corner = corners[0];

    //Sort by key so top edge becomes easy to get to the top.
    let corner_edges = edges.iter().filter(|b| b.id == start_corner).sorted_by_key(|e| e.side.to_index()).collect::<Vec<&BorderId>>();
    let top_edge = corner_edges[0];
    let start_edge = corner_edges[1];

    //Rotate picture according to orientations computed from the location of the edges
    let orientation = get_orientation(top_edge.side, start_edge.side);
    let rotated = orientation.iter()
        .fold(pictures.get(&start_corner).unwrap().pixels.clone(), |p, o| orientate(p, *o));

    //Aggregation for pictures split by column. Easiest for computation.
    let mut picture_cols: Vec<Vec<Grid<char>>> = Vec::new();
    for _i in 0..dimension {
        picture_cols.push(Vec::new());
    }
    picture_cols[0].push(rotated.clone());

    //The id of the last column of the previous picture
    let mut last_col_id = start_edge.opposite();
    //The actual col
    let mut last_col = rotated.iter_col(rotated.cols() - 1).map(|c| *c).collect_vec();
    //Ids of the top of each column, required for later
    let mut ids = Vec::new();
    ids.push(start_corner);

    //Setup initial row
    for i in 1..dimension {
        //Get the matching picture and try all rotations until it matches
        let matched = matches.get(&last_col_id).unwrap()[0];
        let matched_grid = pictures.get(&matched.id).unwrap().pixels.clone();
        for orientation in possible_orientations() {
            let rotate = orientation.iter().fold(matched_grid.clone(), |p, o| orientate(p, *o));
            let first_col = rotate.iter_col(0).map(|c| *c).collect_vec();

            //Once it matches update all variables for the next round
            if first_col == last_col {
                last_col = rotate.iter_col(rotate.cols() -1).map(|c| *c).collect_vec();
                last_col_id = matched.opposite();
                picture_cols[i].push(rotate);
                ids.push(matched.id);
                break;
            }
        }
    }

    //For each column we will get the information from the top row and then expand the column downwards
    for i in 0..dimension {
        let old_pic = &picture_cols[i][0];
        let mut last_row = old_pic.iter_row(old_pic.rows() - 1).map(|c| *c).collect_vec();
        let id = ids[i];
        let side = get_side(&last_row, border_map.get(&id).unwrap()).unwrap();
        let mut last_row_id = BorderId {id, side};
        for _j in 1..dimension {
            let matched = matches.get(&last_row_id).unwrap()[0];
            let matched_grid = pictures.get(&matched.id).unwrap().pixels.clone();
            for orientation in possible_orientations() {
                let rotate = orientation.iter().fold(matched_grid.clone(), |p, o| orientate(p, *o));
                let first_row = rotate.iter_row(0).map(|c| *c).collect_vec();
                if first_row == last_row {
                    last_row = rotate.iter_row(rotate.rows() - 1).map(|c| *c).collect_vec();
                    last_row_id = matched.opposite();
                    picture_cols[i].push(rotate);
                    break;
                }
            }
        }
    }

    //Construct the final picture without borders from the correctly orientated pictures.
    let mut picture = Vec::new();
    for ygrid in 0..dimension {
        for j in 1..picture_size -1 {
            let mut picture_row = Vec::new();
            for i in 0..dimension {
                for k in 1..picture_size - 1 {
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

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
enum Orientation {
    FlipVertical,
    FlipHorizontal,
    RotateLeft,
    RotateRight
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
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn example() {
        assert_eq!(273, picture_str(include_str!("example1")).unwrap())
    }

    #[test]
    pub fn part2_answer() {
        assert_eq!(2002, picture_str(default_input()).unwrap())
    }
}