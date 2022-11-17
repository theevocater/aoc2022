use reqwest;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn read_token() -> String {
    let path = Path::new("TOKEN");
    let mut file = match File::open(&path) {
        Err(why) => panic!("Unable to read {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let mut token = String::new();
    match file.read_to_string(&mut token) {
        Err(why) => panic!("unable to read {}: {}", path.display(), why),
        Ok(_) => (),
    };
    token
}

fn create_day_mains(day: u32) -> std::io::Result<()> {
    let orig = "src/bin/template.rs";
    fs::copy(orig, format!("src/bin/day{}p1.rs", day))?;
    fs::copy(orig, format!("src/bin/day{}p2.rs", day))?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Please pass a day")
    }

    let day: u32 = match args[1].parse() {
        Err(why) => panic!("unable to parse {}", why),
        Ok(day) => day,
    };
    // Make dayNp1 and dayNp2
    match create_day_mains(day) {
        Err(why) => panic!("Unable to copy file: {}", why),
        Ok(_) => (),
    };
    // download todays input.txt to data/dayN.txt
    let token = read_token();
    print!("Token: {}", token);
    let url = format!("https://adventofcode.com/2021/day/{}/input", day);
    let cookie = format!("session={}", token);
    let client = reqwest::blocking::Client::new();
    let resp = match client.get(&url).header("Cookie", cookie).send() {
        Err(why) => panic!("unable to talk to {}: {}", url, why),
        Ok(resp) => resp,
    };
    print!("{}", resp.text().unwrap());
}
