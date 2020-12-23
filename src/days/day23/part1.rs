use crate::days::day23::{parse_input, default_input};

pub fn run() {
    println!("{}", circle_str(default_input()).unwrap())
}

pub fn circle_str(input : &str) -> Result<String, ()> {
    circle(parse_input(input))
}

pub fn circle(mut circle : Vec<i64>) -> Result<String, ()> {
    let max = *circle.iter().max().unwrap();
    let min = *circle.iter().min().unwrap();
    let mut curr = circle[0];
    let mut curr_pos = 0;
    for _i in 0..100 {
        let mut remove_index = (curr_pos + 1) % circle.len();
        let cup0 = circle.remove(remove_index);
        remove_index = remove_index % circle.len();
        let cup1 = circle.remove(remove_index);
        remove_index = remove_index % circle.len();
        let cup2 = circle.remove(remove_index);
        let mut dest = curr - 1;
        if dest == min - 1 {
            dest = max;
        }
        while (dest == cup0) | (dest == cup1) | (dest == cup2) {
            dest = dest - 1;
            if dest == min - 1 {
                dest = max;
            }
        }
        let dest_pos = circle.iter().position(|p| *p == dest).unwrap();
        circle.insert(dest_pos + 1, cup2);
        circle.insert(dest_pos + 1, cup1);
        circle.insert(dest_pos + 1, cup0);
        curr_pos = (circle.iter().position(|p| *p == curr).unwrap() + 1) % circle.len();
        curr = circle[curr_pos];
    }
    let pos1 = circle.iter().position(|p| *p == 1).unwrap();
    let mut pos = pos1 + 1;
    if pos == circle.len() {
        pos = 0;
    }
    let mut res = String::new();
    while pos != pos1 {
        res = format!("{}{}", res, circle[pos]);
        pos = (pos + 1) % circle.len();
        if pos == circle.len() {
            pos = 0;
        }
    }
    Ok(res)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn example() {
        assert_eq!(circle_str("389125467").unwrap(), "67384529")
    }

    #[test]
    pub fn part1_answer() {
        assert_eq!(circle_str(default_input()).unwrap(), "98645732")
    }
}
