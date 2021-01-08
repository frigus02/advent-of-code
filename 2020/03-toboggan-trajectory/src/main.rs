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

    let slopes = &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut result: u64 = 1;
    for slope in slopes {
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

        println!("Right {}, down {}: {}", slope.0, slope.1, trees);
        result *= trees;
    }

    println!("{}", result);
    Ok(())
}
