use std::io::{self, BufRead as _};

type BoxError = Box<dyn std::error::Error>;

fn main() -> Result<(), BoxError> {
    let map: Vec<Vec<_>> = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| if c == '.' { 0 } else { 1 })
                .collect()
        })
        .collect();
    let map_width = map[0].len();
    let map_height = map.len();

    let slope = (3, 1);
    let mut col = 0;
    let mut row = 0;
    let mut trees = 0;
    loop {
        col = (col + slope.0) % map_width;
        row += slope.1;
        if row < map_height {
            trees += map[row][col];
        } else {
            break;
        }
    }

    println!("{}", trees);
    Ok(())
}
