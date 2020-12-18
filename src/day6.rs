use std::collections::HashMap;
use std::collections::HashSet;

pub fn part1(lines: &Vec<String>) -> usize {
    let mut answers = HashSet::new();
    let mut sum = 0;

    for line in lines {
        if line == "" {
            sum += answers.len();
            answers.clear();
            continue;
        }

        for c in line.chars() {
            answers.insert(c);
        }
    }

    sum
}

pub fn part2(lines: &Vec<String>) -> usize {
    let mut answers: HashMap<char, u32> = HashMap::new();
    let mut sum = 0;
    let mut members = 0;

    for line in lines {
        if line == "" {
            let everyone = answers.iter().filter(|(_, &v)| v == members).count();
            sum += everyone;
            answers.clear();
            members = 0;
            continue;
        }

        for c in line.chars() {
            if !answers.contains_key(&c) {
                answers.insert(c, 0);
            }

            answers.insert(c, *answers.get(&c).unwrap() + 1);
        }
        members += 1;
    }

    sum
}
