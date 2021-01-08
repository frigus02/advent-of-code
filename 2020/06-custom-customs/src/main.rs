use std::collections::HashSet;
use std::io::{self, BufRead as _};

type BoxError = Box<dyn std::error::Error>;

fn main() -> Result<(), BoxError> {
    let mut group = HashSet::new();
    let mut total = 0;
    for line in io::stdin().lock().lines() {
        let line = line?;
        if line.is_empty() {
            total += group.len();
            group.clear();
            continue;
        }

        for answer in line.chars() {
            group.insert(answer);
        }
    }

    total += group.len();

    println!("{}", total);
    Ok(())
}
