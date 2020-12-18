use regex::{Captures, Regex};

struct Policy<'a> {
    low: usize,
    high: usize,
    letter: char,
    password: &'a str,
}

impl<'a> Policy<'a> {
    fn new(caps: &'a Captures) -> Self {
        Policy {
            low: Policy::get_match(&caps, 1).parse::<usize>().unwrap(),
            high: Policy::get_match(&caps, 2).parse::<usize>().unwrap(),
            letter: Policy::get_match(&caps, 3).chars().next().unwrap(),
            password: Policy::get_match(&caps, 4),
        }
    }

    fn is_valid(&self) -> bool {
        let count = self.password.matches(self.letter).count();
        count >= self.low && count <= self.high
    }

    fn get_match(capture: &'a Captures, index: usize) -> &'a str {
        capture.get(index).unwrap().as_str()
    }

    fn is_valid2(&self) -> bool {
        let mut chars = self.password.chars();
        let diff = self.high - self.low - 1;
        let first = chars.nth(self.low - 1).unwrap();
        let second = chars.nth(diff).unwrap();

        (self.letter == first) ^ (self.letter == second)
    }
}

pub fn run(lines: &Vec<String>) -> u32 {
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    let mut valid = 0;
    for line in lines {
        let caps = re.captures(line).unwrap();
        let policy = Policy::new(&caps);
        if policy.is_valid() {
            valid += 1;
        }
    }

    valid
}

pub fn part2(lines: &Vec<String>) -> u32 {
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    let mut valid = 0;
    for line in lines {
        let caps = re.captures(line).unwrap();
        let policy = Policy::new(&caps);
        if policy.is_valid2() {
            valid += 1;
        }
    }

    valid
}
