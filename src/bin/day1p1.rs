use std::io;
use std::io::BufRead;

fn main() {
    let inputs: Vec<String> = io::stdin().lock().lines().flatten().collect();
    println!(
        "{}",
        inputs
            .split(|elem| elem == "")
            .map(|group| group.iter().map(|v| v.parse::<i32>().unwrap()).sum::<i32>())
            .max()
            .unwrap()
    );
}
