use std::io::{self, BufRead as _};

type BoxError = Box<dyn std::error::Error>;

fn main() -> Result<(), BoxError> {
    let max = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let seat = line.expect("valid seat");
            let (row, _) = seat[0..7].chars().fold((0, 127), |(low, high), c| match c {
                'B' => (high - (high - low) / 2, high),
                'F' => (low, high - (high - low) / 2),
                _ => panic!("invalid row"),
            });
            let (col, _) = seat[7..10].chars().fold((0, 7), |(low, high), c| match c {
                'R' => (high - (high - low) / 2, high),
                'L' => (low, high - (high - low) / 2),
                _ => panic!("invalid col"),
            });
            row * 8 + col
        })
        .max()
        .expect("at least one seat");

    println!("{}", max);
    Ok(())
}
