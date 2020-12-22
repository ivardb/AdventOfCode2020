use crate::days::day21::{FoodLine, parse_input, default_input};
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

pub fn run() {
    println!("{}", food_str(default_input()).unwrap())
}

pub fn food_str(input : &str) -> Result<String, ()> {
    food(parse_input(input))
}

pub fn food(input : Vec<FoodLine>) -> Result<String, ()> {
    let mut allergen_map : HashMap<&String, Vec<HashSet<String>>> = HashMap::new();
    for food in &input {
        for allergen in &food.allergens {
            allergen_map.entry(allergen).or_insert(Vec::new()).push(food.ingredients.clone());
        }
    }
    let mut constraints: HashMap<_, _>= allergen_map.iter()
        .map(|(k, v)| {
            let intersection = v.iter().skip(1).fold(v[0].clone(), |acc, set| acc.intersection(set).map(|s| s.clone()).collect());
            (k, intersection)
        }).collect();
    let mut mappings = HashMap::new();
    let final_length = constraints.len();
    while mappings.len() < final_length {
        let mut match_rule = None;
        let mut match_value = None;
        for (rule, values) in &constraints {
            if values.len() == 1 {
                match_rule = Some(rule.clone());
                match_value = Some(values.iter().next().unwrap().clone());
                break;
            }
        }
        constraints.remove(&match_rule.unwrap());
        constraints = constraints.iter()
            .map(|(k, v)| {
                (*k, v.iter().cloned().filter(|n| *n != match_value.as_ref().unwrap().clone()).collect())
            })
            .collect();
        mappings.insert(match_rule.unwrap(), match_value.unwrap());
    }
    Ok(mappings.iter()
        .sorted_by_key(|(k, _v)| k.clone())
        .map(|(_k,v)| v).join(","))
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn part2_answer() {
        assert_eq!(food_str(default_input()).unwrap(), "fqhpsl,zxncg,clzpsl,zbbnj,jkgbvlxh,dzqc,ppj,glzb")
    }
}