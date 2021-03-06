use crate::days::day23::{parse_input, default_input, Node};
use std::collections::HashMap;

pub fn run() {
    println!("{}", circle_str(default_input()).unwrap())
}

pub fn circle_str(input : &str) -> Result<String, ()> {
    circle(parse_input(input))
}

pub fn circle(circle : Vec<i64>) -> Result<String, ()> {
    let nodes: Vec<Node<i64>> = circle.iter().map(|n| Node::new(*n)).collect();
    let mut nxt = HashMap::new();
    for i in 0..circle.len() -1 {
        nxt.insert(&nodes[i], &nodes[i+1]);
    }
    nxt.insert(&nodes[circle.len() - 1], &nodes[0]);
    let max = *circle.iter().max().unwrap();
    let min = *circle.iter().min().unwrap();
    let mut curr = &nodes[0];
    for _i in 0..100 {
        let removed = [nxt.get(curr).unwrap().clone(), nxt.get(nxt.get(curr).unwrap()).unwrap().clone(), nxt.get(nxt.get(nxt.get(curr).unwrap()).unwrap()).unwrap().clone()];
        nxt.insert(curr, nxt.get(removed[2]).unwrap());
        let mut dest = curr.val - 1;
        if dest == min - 1 {
            dest = max;
        }
        while removed.iter().any(|n| n.val == dest) {
            dest = dest - 1;
            if dest == min - 1 {
                dest = max;
            }
        }
        let dest_pos = circle.iter().position(|p| *p == dest).unwrap();
        let dest = &nodes[dest_pos];
        nxt.insert(removed[2], nxt.get(dest).unwrap());
        nxt.insert(dest, removed[0]);
        curr = nxt.get(curr).unwrap();
    }
    let pos1 = nodes.iter().position(|p| p.val == 1).unwrap();
    let node1 = &nodes[pos1];
    let mut nxt_node = nxt.get(node1).unwrap().clone();
    let mut res = String::new();
    while nxt_node.val != 1 {
        res = format!("{}{}", res, nxt_node.val);
        nxt_node = nxt.get(nxt_node).unwrap().clone();
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
