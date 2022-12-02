use std::io;
use std::io::BufRead;

fn map_value(thing: &str) -> i32 {
    match thing {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => 10,
    }
}

fn calculate_match(them: &str, you: &str) -> i32 {
    let combo: &str = &format!("{}{}", them, you);
    let val = match combo {
        "AX" => 3,
        "AY" => 6,
        "AZ" => 0,
        "BX" => 0,
        "BY" => 3,
        "BZ" => 6,
        "CX" => 6,
        "CY" => 0,
        "CZ" => 3,
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
