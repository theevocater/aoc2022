use std::collections::HashSet;
use std::io;
use std::io::BufRead;

fn main() {
    let inputs: Vec<String> = io::stdin().lock().lines().flatten().collect();
    let window_size = 14;
    inputs.iter().for_each(|input| {
        println!(
            "{:?}",
            input
                .chars()
                .enumerate()
                .collect::<Vec<(usize, char)>>()
                .windows(window_size)
                .find(|window| {
                    let mut s = HashSet::new();
                    window.iter().for_each(|a| {
                        s.insert(a.1);
                    });
                    s.len() == window_size
                })
        );
    })
}
