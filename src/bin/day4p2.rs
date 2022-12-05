use std::io;
use std::io::BufRead;

fn calculate(ls: i32, le: i32, rs: i32, re: i32) -> bool {
    (ls <= rs && le >= rs) || (rs <= ls && re >= ls)
}

fn main() {
    let inputs: Vec<String> = io::stdin().lock().lines().flatten().collect();
    println!(
        "{}",
        inputs
            .iter()
            .map(|line| {
                let split = line
                    .splitn(2, ',')
                    .map(|pair| pair.splitn(2, '-'))
                    .flatten()
                    .map(|c| c.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                calculate(split[0], split[1], split[2], split[3])
            })
            .filter(|f| *f)
            .count()
    )
}
