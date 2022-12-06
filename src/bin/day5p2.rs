use anyhow::Context;
use std::fmt;
use std::io;
use std::io::BufRead;

struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(amount: {}, from: {}, to: {})",
            self.amount, self.from, self.to
        )
    }
}

fn main() -> anyhow::Result<()> {
    let inputs: Vec<String> = io::stdin().lock().lines().flatten().collect();
    // number of stacks is line 1 len / 4 rounded up
    let num_stacks = (inputs.first().context("failed to parse")?.len() + 1) / 4;
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); num_stacks];
    let mut moves: Vec<Move> = Vec::new();
    inputs
        .iter()
        .for_each(|input| match input.trim().chars().nth(0) {
            // If starts with [ [T] stacks each take up 4 chars and include trailing spaces subdivide the
            // stacks by 4
            Some('[') => input
                .chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .enumerate()
                .for_each(|(index, a)| {
                    if a[1] != ' ' {
                        stacks[index].insert(0, a[1])
                    }
                }),
            // if line starts with 'm' parse these into a matrix of a list of stacks, and the boxes in each
            // stack, with the top at split the move lines on spaces, use 1,3,5 for amount,from,to then
            // iterate over the moves and mutate the matrix then return the end of each sublist
            Some('m') => {
                let split = input
                    .split(' ')
                    .flat_map(|word| word.parse::<usize>())
                    .collect::<Vec<usize>>();
                moves.push(Move {
                    amount: split[0],
                    from: split[1] - 1, // convert to 0 index
                    to: split[2] - 1,
                });
            }
            // if line starts with space or anything else ignore
            _ => println!("ignoring {}", input),
        });
    stacks.iter().for_each(|s| {
        s.iter().for_each(|c| print!("{} ", c));
        println!("")
    });
    moves.iter().for_each(|m| println!("{}", m));
    for m in moves.iter() {
        println!("Performing {}", m);
        let len = stacks[m.from].len();
        let from = stacks[m.from]
            .drain(len - m.amount..len)
            .collect::<Vec<char>>(); // I bet there is more concise to do this, but I don't know how to borrow this correctly.
        for i in from.iter() {
            stacks[m.to].push(*i)
        }
    }
    stacks.iter().for_each(|s| {
        s.iter().for_each(|c| print!("{} ", c));
        println!("")
    });
    stacks.iter().for_each(|s| {
        print!("{}", s[s.len() - 1]);
    });
    println!("");
    Ok(())
}
