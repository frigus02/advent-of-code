use regex::Regex;
use std::io::{self, BufRead as _};

type BoxError = Box<dyn std::error::Error>;

fn main() -> Result<(), BoxError> {
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").expect("regex is valid");
    let mut valid = 0;
    for line in io::stdin().lock().lines() {
        let line = line?;
        let caps = re.captures(&line).ok_or("invalid line")?;
        let min: usize = caps.get(1).ok_or("invalid line")?.as_str().parse()?;
        let max: usize = caps.get(2).ok_or("invalid line")?.as_str().parse()?;
        let ch = caps
            .get(3)
            .ok_or("invalid line")?
            .as_str()
            .chars()
            .next()
            .ok_or("invalid line")?;
        let password = caps.get(4).ok_or("invalid line")?.as_str();
        let count = password.chars().filter(|c| c == &ch).count();
        if min <= count && count <= max {
            valid += 1;
        }
    }

    println!("{}", valid);
    Ok(())
}
