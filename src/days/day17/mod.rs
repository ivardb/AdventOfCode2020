use std::collections::HashSet;

pub mod part1;
pub mod part2;

pub fn run() {
    part1::run();
    part2::run();
}

pub fn default_input() -> &'static str {
    include_str!("input")
}

pub fn parse_input3d(input : &str) -> HashSet<Point3d> {
    let mut active = HashSet::new();
    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c == '#'{
                active.insert(Point3d {x: x as i8, y: y as i8, z: 0});
            }
        }
    }
    active
}

pub fn parse_input4d(input : &str) -> HashSet<Point4d> {
    let mut active = HashSet::new();
    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c == '#'{
                active.insert(Point4d {x: x as i8, y: y as i8, z: 0, w:0});
            }
        }
    }
    active
}

#[derive(Eq, PartialEq, Hash)]
pub struct Point3d {
    pub x: i8,
    pub y: i8,
    pub z: i8
}

impl Point3d {
    pub fn update(&self, dx : i8, dy: i8, dz: i8) -> Point3d {
        Point3d {x: self.x + dx, y: self.y + dy, z: self.z + dz}
    }
}

#[derive(Eq, PartialEq, Hash)]
pub struct Point4d {
    pub x: i8,
    pub y: i8,
    pub z: i8,
    pub w: i8
}

impl Point4d {
    pub fn update(&self, dx : i8, dy: i8, dz: i8, dw: i8) -> Point4d {
        Point4d {x: self.x + dx, y: self.y + dy, z: self.z + dz, w: self.w + dw}
    }
}