use crate::days::day20::{Picture, parse_input, default_input, compute_borders};
use std::collections::HashMap;

pub fn run() {
    println!("{}", picture_str(default_input()).unwrap())
}

pub fn picture_str(input : &str) -> Result<i64, ()> {
    picture(parse_input(input))
}

pub fn picture(pictures : Vec<Picture>) -> Result<i64, ()> {
    let picture_size = pictures[0].pixels.cols();
    let border_map = compute_borders(&pictures, picture_size);

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

    #[test]
    pub fn part1_answer() {
        assert_eq!(18411576553343, picture_str(default_input()).unwrap())
    }
}