use std::io;
use std::io::BufRead;

fn map_value(thing: &str) -> i32 {
    match thing {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => panic!("asdfasdf"),
    }
}

fn calculate_match(them: &str, you: &str) -> i32 {
    let combo: &str = &format!("{}{}", them, you);
    let val = match combo {
        "AX" => 3,
        "AY" => 1,
        "AZ" => 2,
        "BX" => 1,
        "BY" => 2,
        "BZ" => 3,
        "CX" => 2,
        "CY" => 3,
        "CZ" => 1,
        _ => panic!("aphnasdf"),
    };
    val + map_value(you)
}

fn main() {
    let inputs: Vec<String> = io::stdin().lock().lines().flatten().collect();
    println!(
        "{}",
        inputs
            .iter()
            .map(|line| line.splitn(2, " ").collect::<Vec<&str>>())
            .map(|input| calculate_match(input[0], input[1]))
            .collect::<Vec<i32>>()
            .iter()
            .sum::<i32>()
    )
}
