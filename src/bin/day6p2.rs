use std::collections::HashSet;
use std::io;
use std::io::BufRead;

fn main() {
    io::stdin().lock().lines().flatten().for_each(|input| {
        println!(
            "{}",
            input
                .chars()
                .enumerate()
                .collect::<Vec<(usize, char)>>()
                .windows(14)
                .find(|window| {
                    let mut s = HashSet::new();
                    window.iter().all(|a| s.insert(a.1))
                })
                .and_then(|a| a.last().map(|a| a.0 + 1))
                .expect("Couldn't find anything")
        );
    })
}
