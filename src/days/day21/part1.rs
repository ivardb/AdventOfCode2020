use crate::days::day21::{FoodLine, parse_input, default_input};
use std::collections::{HashMap, HashSet};

pub fn run() {
    println!("{}", food_str(default_input()).unwrap())
}

pub fn food_str(input : &str) -> Result<usize, ()> {
    food(parse_input(input))
}

pub fn food(input : Vec<FoodLine>) -> Result<usize, ()> {
    let mut allergen_map : HashMap<&String, Vec<HashSet<String>>> = HashMap::new();
    for food in &input {
        for allergen in &food.allergens {
            allergen_map.entry(allergen).or_insert(Vec::new()).push(food.ingredients.clone());
        }
    }
    let constraints : HashMap<_, _>= allergen_map.iter()
        .map(|(k, v)| {
            let intersection = v.iter().skip(1).fold(v[0].clone(), |acc, set| acc.intersection(set).map(|s| s.clone()).collect());
            (k, intersection)
        }).collect();
    Ok(input.iter().map(|f| f.ingredients.iter().filter(|i| !constraints.iter().any(|(_k, v)| v.contains(*i))).count()).sum())
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn example() {
        assert_eq!(food_str("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)").unwrap(), 5)
    }

    #[test]
    pub fn part1_answer() {
        assert_eq!(food_str(default_input()).unwrap(), 2584)
    }
}