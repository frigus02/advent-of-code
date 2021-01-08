use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead as _};

type BoxError = Box<dyn std::error::Error>;

trait Validation: std::fmt::Debug {
    fn validate(&self, input: &str) -> bool;
}

#[derive(Debug)]
struct ValidLength {
    len: usize,
}

impl ValidLength {
    fn new(len: usize) -> Self {
        Self { len }
    }
}

impl Validation for ValidLength {
    fn validate(&self, input: &str) -> bool {
        input.len() == self.len
    }
}

#[derive(Debug)]
struct ValidNumber {
    min: usize,
    max: usize,
}

impl ValidNumber {
    fn new(min: usize, max: usize) -> Self {
        Self { min, max }
    }
}

impl Validation for ValidNumber {
    fn validate(&self, input: &str) -> bool {
        if let Ok(n) = input.parse() {
            self.min <= n && n <= self.max
        } else {
            false
        }
    }
}

#[derive(Debug)]
struct ValidNumberWithUnit {
    valid_number: ValidNumber,
    unit: &'static str,
}

impl ValidNumberWithUnit {
    fn new(unit: &'static str, min: usize, max: usize) -> Self {
        Self {
            valid_number: ValidNumber::new(min, max),
            unit,
        }
    }
}

impl Validation for ValidNumberWithUnit {
    fn validate(&self, input: &str) -> bool {
        if let Some(number) = input.strip_suffix(self.unit) {
            self.valid_number.validate(number)
        } else {
            false
        }
    }
}

#[derive(Debug)]
struct ValidString {
    options: HashSet<&'static str>,
}

impl ValidString {
    fn new(options: &[&'static str]) -> Self {
        Self {
            options: options.iter().cloned().collect(),
        }
    }
}

impl Validation for ValidString {
    fn validate(&self, input: &str) -> bool {
        self.options.contains(input)
    }
}

#[derive(Debug)]
struct ValidHexColor;

impl Validation for ValidHexColor {
    fn validate(&self, input: &str) -> bool {
        if let Some(hex) = input.strip_prefix("#") {
            hex.len() == 6 && hex.chars().all(|c| c.is_digit(16))
        } else {
            false
        }
    }
}

#[derive(Debug)]
struct And {
    a: Box<dyn Validation>,
    b: Box<dyn Validation>,
}

impl And {
    fn new(a: Box<dyn Validation>, b: Box<dyn Validation>) -> Self {
        Self { a, b }
    }
}

impl Validation for And {
    fn validate(&self, input: &str) -> bool {
        self.a.validate(input) && self.b.validate(input)
    }
}

#[derive(Debug)]
struct Or {
    a: Box<dyn Validation>,
    b: Box<dyn Validation>,
}

impl Or {
    fn new(a: Box<dyn Validation>, b: Box<dyn Validation>) -> Self {
        Self { a, b }
    }
}

impl Validation for Or {
    fn validate(&self, input: &str) -> bool {
        self.a.validate(input) || self.b.validate(input)
    }
}

fn main() -> Result<(), BoxError> {
    let mut required_fields: HashMap<&'static str, Box<dyn Validation>> = HashMap::new();
    required_fields.insert("byr", Box::new(ValidNumber::new(1920, 2002)));
    required_fields.insert("iyr", Box::new(ValidNumber::new(2010, 2020)));
    required_fields.insert("eyr", Box::new(ValidNumber::new(2020, 2030)));
    required_fields.insert(
        "hgt",
        Box::new(Or::new(
            Box::new(ValidNumberWithUnit::new("cm", 150, 193)),
            Box::new(ValidNumberWithUnit::new("in", 59, 76)),
        )),
    );
    required_fields.insert("hcl", Box::new(ValidHexColor));
    required_fields.insert(
        "ecl",
        Box::new(ValidString::new(&[
            "amb", "blu", "brn", "gry", "grn", "hzl", "oth",
        ])),
    );
    required_fields.insert(
        "pid",
        Box::new(And::new(
            Box::new(ValidNumber::new(0, 999999999)),
            Box::new(ValidLength::new(9)),
        )),
    );

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
            .filter_map(|field| {
                let mut parts = field.splitn(2, ':');
                if let (Some(name), Some(value)) = (parts.next(), parts.next()) {
                    Some((name, value))
                } else {
                    None
                }
            })
            .filter_map(|(name, value)| {
                required_fields
                    .get(name)
                    .map(|validation| (validation, value))
            })
            .filter(|(validation, value)| validation.validate(value))
            .count();
    }

    println!("{}", valid);
    Ok(())
}
