use crate::days::day24::{Direction, default_input, parse_input, Point};
use std::collections::{HashSet, HashMap};

pub fn run() {
    println!("{}", hex_str(default_input()).unwrap());
}

pub fn hex_str(input : &str) -> Result<usize, ()> {
    hex(parse_input(input))
}

pub fn hex(input : Vec<Vec<Direction>>) -> Result<usize, ()> {
    //Setup tiles
    let mut black = HashSet::new();
    for directions in input {
        let mut point = Point {x: 0, y:0, z:0};
        for direction in directions {
            point = point.add(direction.to_point());
        }
        if black.contains(&point) {
            black.remove(&point);
        } else {
            black.insert(point);
        }
    }

    for _i in 0..100 {
        let mut neighbours: HashMap<Point, u8> = HashMap::new();
        for p in &black {
            for dir in Direction::all() {
                neighbours.entry(p.add(dir.to_point())).and_modify(|c| *c += 1).or_insert(1);
            }
        }
        let mut new_black = HashSet::new();
        for (p, c) in neighbours {
            if (c == 1) && black.contains(&p) {
                new_black.insert(p);
            } else if c == 2 {
                new_black.insert(p);
            }
        }
        black = new_black;
    }
    Ok(black.len())
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn example() {
        assert_eq!(hex_str("sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew").unwrap(), 2208)
    }

    #[test]
    pub fn part2_answer() {
            assert_eq!(hex_str(default_input()).unwrap(), 3869)
        }
}