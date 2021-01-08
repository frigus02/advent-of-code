use std::io::{self, BufRead as _};

type BoxError = Box<dyn std::error::Error>;

fn main() -> Result<(), BoxError> {
    let mut seats: Vec<_> = io::stdin()
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
        .collect();
    seats.sort_unstable();
    let min = seats[0];
    let max = seats[seats.len() - 1];
    let (_, after_mine) = seats
        .into_iter()
        .enumerate()
        .find(|(i, seat)| i + min < *seat)
        .expect("my seat");
    let my = after_mine - 1;

    println!("min: {}", min);
    println!("max: {}", max);
    println!("my: {}", my);
    Ok(())
}
