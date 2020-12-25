use crate::days::day25::{Keys, parse_input, default_input};

pub fn run() {
    println!("{}", room_str(default_input()).unwrap())
}

pub fn room_str(input : &str) -> Result<i64, ()> {
    room(parse_input(input))
}

pub fn room(keys : Keys) -> Result<i64, ()> {
    let mut value = 1;
    let mut loop_size = 0;
    while value != keys.card {
        value = transform(value, 7);
        loop_size += 1;
    }
    value = 1;
    for _i in 0..loop_size {
        value = transform(value, keys.door)
    }
    Ok(value)
}

fn transform(value: i64, subject: i64) -> i64 {
    (value * subject) % 20201227
}