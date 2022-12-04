use std::collections::{HashMap, HashSet};
use std::io;
use std::io::BufRead;

fn main() {
    let inputs: Vec<String> = io::stdin().lock().lines().flatten().collect();
    let mut sum_map = HashMap::new();
    sum_map.extend(('a'..='z').chain('A'..='Z').zip(1..));
    println!(
        "{}",
        inputs
            .iter()
            .map(|line| {
                let mut uniq = HashSet::new();
                line.chars().enumerate().find(|(index, letter)| {
                    if index < &(line.len() / 2) {
                        uniq.insert(letter.clone());
                        false
                    } else {
                        uniq.contains(letter)
                    }
                })
            })
            .flat_map(|a| {
                let (_index, letter) = a?;
                sum_map.get(&letter)
            })
            .sum::<i32>()
    )
}
