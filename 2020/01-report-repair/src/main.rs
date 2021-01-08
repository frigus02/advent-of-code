use std::collections::HashSet;
use std::io::{self, BufRead as _};

type BoxError = Box<dyn std::error::Error>;

fn main() -> Result<(), BoxError> {
    let mut seen = HashSet::new();
    for line in io::stdin().lock().lines() {
        let n: u32 = line?.parse()?;
        let need = 2020 - n;
        if seen.contains(&need) {
            println!("{} * {} = {}", n, need, n * need);
            return Ok(());
        } else {
            seen.insert(n);
        }
    }

    println!("No result");
    Ok(())
}
