use reqwest;
use reqwest::header;
use std::env;
use std::fs;
use std::path::Path;

fn read_token() -> anyhow::Result<String> {
    let path = Path::new("TOKEN");
    let token = match fs::read_to_string(&path) {
        Err(why) => panic!("Unable to read {}: {}", path.display(), why),
        Ok(file) => file,
    };
    Ok(token.trim().to_string())
}

fn create_day_mains(day: u32) -> std::io::Result<()> {
    let orig = "src/bin/template.rs";
    fs::copy(orig, format!("src/bin/day{}p1.rs", day))?;
    fs::copy(orig, format!("src/bin/day{}p2.rs", day))?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Please pass a day")
    }

    let day: u32 = args[1].parse()?;

    // Make dayNp1 and dayNp2
    create_day_mains(day)?;

    // download todays input.txt to data/dayN.txt
    let token = read_token();
    let url = format!("https://adventofcode.com/2022/day/{}/input", day);
    let cookie = format!("session={}", token?);
    let client = reqwest::blocking::Client::new();
    let resp = client
        .get(&url)
        .header(
            header::HeaderName::from_static("cookie"),
            header::HeaderValue::from_str(&cookie).expect("failed to parse header"),
        )
        .send()?;

    let resp = resp.text().unwrap();
    let input_file = format!("input_day{}.txt", day);
    fs::write(input_file, resp)?;
    Ok(())
}
