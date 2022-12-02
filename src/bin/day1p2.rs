use std::collections::BinaryHeap;
use std::io;
use std::io::BufRead;

fn main() {
    let inputs: Vec<String> = io::stdin().lock().lines().flatten().collect();
    let mut output = inputs
        .split(|elem| elem == "")
        .map(|group| group.iter().map(|v| v.parse::<i32>().unwrap()).sum::<i32>())
        .collect::<BinaryHeap<i32>>();
    let sum = output.pop().unwrap() + output.pop().unwrap() + output.pop().unwrap();
    println!("{}", sum);
}
