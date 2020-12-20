use crate::days::day20::{Picture, parse_input, default_input};
use std::collections::HashMap;

pub fn run() {
    println!("{}", picture_str(default_input()).unwrap())
}

pub fn picture_str(input : &str) -> Result<i64, ()> {
    picture(parse_input(input))
}

pub fn picture(pictures : Vec<Picture>) -> Result<i64, ()> {
    let picture_width = pictures[0].pixels.cols();
    let picture_height = pictures[0].pixels.rows();
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
    let mut edges : HashMap<i64, Vec<Vec<char>>> = HashMap::new();
    for picture in &pictures {
        for border in border_map.get(&picture.id).unwrap() {
            let mut reverse = border.clone();
            reverse.reverse();
            let edge = !pictures.iter().filter(|p| p.id != picture.id)
                .map(|p| border_map.get(&p.id).unwrap())
                .any(|borders| borders.contains(&reverse) | borders.contains(border));
            if edge {
                edges.entry(picture.id)
                    .and_modify(|v| v.push(border.clone()))
                    .or_insert({
                        let mut v = Vec::new();
                        v.push(border.clone());
                        v
                    });
            }
        }
    }
    let corners : Vec<_> = edges.iter().filter(|(_k,v)| v.len() == 2 ).map(|(k,_v)| *k).collect();
    Ok(corners.iter().product())
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn example() {
        assert_eq!(20899048083289, picture_str(include_str!("example1")).unwrap())
    }
}