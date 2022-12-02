use std::io;
use std::io::BufRead;

fn main() {
    let inputs: Vec<String> = io::stdin().lock().lines().flatten().collect();
    let mut output = inputs
        .split(|elem| elem == "")
        .map(|group| group.iter().map(|v| v.parse::<i32>().unwrap()).sum::<i32>())
        .collect::<Vec<i32>>();
    output.sort();
    output.reverse();
    println!("{}", output.iter().take(3).sum::<i32>());
}
