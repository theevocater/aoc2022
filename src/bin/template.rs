use std::io;
use std::io::BufRead;

fn main() {
    let inputs: Vec<String> = io::stdin().lock().lines().flatten().collect();
    inputs.iter().for_each(|input| println!("{}", input))
}
