use std::io;
use std::io::BufRead;

fn main() {
    let inputs: Vec<String> = io::stdin().lock().lines().flatten().collect();
    for input in inputs.iter() {
        println!("{}", input)
    }
}
