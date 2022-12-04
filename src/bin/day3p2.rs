use std::collections::{HashMap, HashSet};
use std::io;
use std::io::BufRead;

fn main() {
    let inputs: Vec<String> = io::stdin().lock().lines().flatten().collect();
    let sum_map: HashMap<char, i32> = ('a'..='z').chain('A'..='Z').zip(1..).collect();
    println!(
        "{}",
        inputs
            .chunks(3)
            .flat_map(|lines| {
                lines
                    .iter()
                    .map(|a| a.chars().collect::<HashSet<char>>())
                    .reduce(|a, b| a.intersection(&b).cloned().collect::<HashSet<char>>())
                    .unwrap()
                    .drain()
                    .next()
            })
            .flat_map(|letter| sum_map.get(&letter))
            .sum::<i32>()
    )
}
