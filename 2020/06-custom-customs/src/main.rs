use std::collections::HashMap;
use std::io::{self, BufRead as _};

type BoxError = Box<dyn std::error::Error>;

fn main() -> Result<(), BoxError> {
    let mut group_answers = HashMap::new();
    let mut group_people = 0;
    let mut total = 0;
    for line in io::stdin().lock().lines() {
        let line = line?;
        if line.is_empty() {
            total += group_answers
                .drain()
                .filter(|(_, v)| *v == group_people)
                .count();
            group_people = 0;
            continue;
        }

        group_people += 1;
        for answer in line.chars() {
            let count = group_answers.entry(answer).or_insert(0);
            *count += 1;
        }
    }

    total += group_answers
        .drain()
        .filter(|(_, v)| *v == group_people)
        .count();

    println!("{}", total);
    Ok(())
}
