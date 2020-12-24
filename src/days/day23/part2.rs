use crate::days::day23::{parse_input, default_input, Node};
use std::collections::HashMap;

pub fn run() {
    println!("{}", circle_str(default_input()).unwrap())
}

pub fn circle_str(input : &str) -> Result<i64, ()> {
    let mut vec = parse_input(input);
    let mut lengthening: Vec<_> = (input.len()+1..1000001).map(|n| n as i64).collect();
    vec.append(&mut lengthening);
    circle(vec)
}

pub fn circle(circle : Vec<i64>) -> Result<i64, ()> {
    let nodes: Vec<Node<i64>> = circle.iter().map(|n| Node::new(*n)).collect();
    let mut nxt = HashMap::new();
    for i in 0..circle.len() -1 {
        nxt.insert(&nodes[i], &nodes[i+1]);
    }
    nxt.insert(&nodes[circle.len() - 1], &nodes[0]);
    let lookup : HashMap<_,_> = nodes.iter().map(|n| (n.val, n)).collect();

    let max = *circle.iter().max().unwrap();
    let min = *circle.iter().min().unwrap();
    let mut curr = &nodes[0];
    for _i in 0..10000000 {
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
        let dest = lookup.get(&dest).unwrap().clone();
        nxt.insert(removed[2], nxt.get(dest).unwrap());
        nxt.insert(dest, removed[0]);
        curr = nxt.get(curr).unwrap();
    }
    let node1 = lookup.get(&1).unwrap().clone();
    let nxt_node = nxt.get(node1).unwrap().clone();
    let nxt_nxt_node = nxt.get(nxt_node).unwrap().clone();
    Ok(nxt_node.val * nxt_nxt_node.val)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[ignore]
    #[test]
    pub fn part2_answer() {
        assert_eq!(circle_str(default_input()).unwrap(), 689500518476)
    }
}
