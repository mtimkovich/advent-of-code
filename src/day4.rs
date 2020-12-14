use regex::Regex;
// use std::collections::HashMap;
use std::collections::HashSet;

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

// pub fn part2(lines: &Vec<String>) -> u32 {
//     let re = Regex::new(r"([a-z]+):([^\s]+)").unwrap();
//     let mut fields = HashMap::new();
//     let mut valid = 0;

//     for line in lines {
//         if line == "" {
//             let passport = Passport::new(&fields);
//             // match passport {
//                 // Some(_) => valid += 1,
//                 // None => (),
//             // }

//             fields.clear();
//         }

//         for cap in re.captures_iter(&line) {
//             fields.insert(
//                 cap[1].to_owned(),
//                 cap[2].to_owned());
//         }
//     }

//     valid
// }
