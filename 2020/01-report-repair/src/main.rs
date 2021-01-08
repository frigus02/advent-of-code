use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead as _};

type BoxError = Box<dyn std::error::Error>;

fn main() -> Result<(), BoxError> {
    let mut one = HashSet::new();
    let mut two = HashMap::new();
    for line in io::stdin().lock().lines() {
        let n: u32 = line?.parse()?;
        let need = 2020 - n;
        if let Some((a, b)) = two.get(&need) {
            println!("{} * {} * {} = {}", a, b, n, a * b * n);
            return Ok(());
        } else {
            for a in &one {
                two.insert(*a + n, (*a, n));
            }

            one.insert(n);
        }
    }

    println!("No result");
    Ok(())
}
