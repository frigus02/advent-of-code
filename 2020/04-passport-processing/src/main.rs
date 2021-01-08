use std::collections::HashSet;
use std::io::{self, BufRead as _};

type BoxError = Box<dyn std::error::Error>;

fn main() -> Result<(), BoxError> {
    let required_fields: HashSet<&'static str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .cloned()
        .collect();

    let mut valid = 0;
    let mut current_passport_fields = 0;
    for line in io::stdin().lock().lines() {
        let line = line?;
        if line.is_empty() {
            if current_passport_fields >= required_fields.len() {
                valid += 1;
            }

            current_passport_fields = 0;
        }

        current_passport_fields += line
            .split(' ')
            .filter_map(|field| field.splitn(2, ':').next())
            .filter(|field_name| required_fields.contains(field_name))
            .count();
    }

    println!("{}", valid);
    Ok(())
}
