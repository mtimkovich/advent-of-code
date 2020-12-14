use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

union FieldValue<'a> {
    year: u32,
    height: &'a str,
    hcl: &'a str,
    ecl: &'a str,
    pid: u32,
}

fn has_required(set: &HashSet<String>) -> bool {
    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    required_fields.into_iter().all(|i| set.contains(i))
}

pub fn part1(lines: &Vec<String>) -> u32 {
    let re = Regex::new(r"([a-z]+):[^\s]+").unwrap();
    let mut fields = HashSet::new();
    let mut valid = 0;

    for line in lines {
        if line == "" {
            if has_required(&fields) {
                valid += 1;
            }
            fields.clear();
        }

        for cap in re.captures_iter(&line) {
            fields.insert(cap[1].to_owned());
        }
    }

    valid
}

fn validate_year(name: &str, value: &str) -> bool {
    let valid_years: HashMap<&str, (u32, u32)> = [
        ("byr", (1920, 2002)),
        ("iyr", (2010, 2020)),
        ("eyr", (2020, 2030)),
    ].iter().cloned().collect();
    let year = match value.parse::<u32>() {
        Ok(y) => y,
        Err(_) => return false,
    };

    let (low, high) = valid_years.get(name).unwrap();
    year >= *low && year <= *high
}

fn validate_height(value: &str) -> bool {
    let re = Regex::new(r"(\d+)(cm|in)").unwrap();
    let caps = match re.captures(value) {
        Some(c) => c,
        None => return false,
    };

    let mag = match &caps[1].parse::<u32>() {
        Ok(v) => v.clone(),
        Err(_) => return false,
    };

    match &caps[2] {
        "cm" => mag >= 150 && mag <= 193,
        "in" => mag >= 59 && mag <= 76,
        _ => false,
    }
}

fn validate_field(name: &str, value: &str) -> bool {
    if name == "byr" || name == "iyr" || name == "eyr" {
        return validate_year(name, value);
    } else if name == "pid" {
        if value.len() != 9 {
            return false;
        }
        return value.parse::<u32>().is_ok();
    } else if name == "ecl" {
        let ecls: HashSet<&str> = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            .iter().cloned().collect();
        return ecls.contains(value);
    } else if name == "hcl" {
        let re = Regex::new(r"#[a-z0-9]{6}").unwrap();
        return re.is_match(value);
    } else if name == "hgt" {
        return validate_height(&value);
    }

    true
}

pub fn part2(lines: &Vec<String>) -> u32 {
    let re = Regex::new(r"([a-z]+):([^\s]+)").unwrap();
    let mut fields = HashSet::new();
    let mut valid_count = 0;
    let mut is_valid = true;

    for line in lines {
        if line == "" {
            if is_valid && has_required(&fields) {
                valid_count += 1;
            }
            fields.clear();
            is_valid = true;
        }

        for cap in re.captures_iter(&line) {
            if is_valid && !validate_field(&cap[1], &cap[2]) {
                is_valid = false;
            }
            fields.insert(cap[1].to_owned());
        }
    }

    valid_count
}
