use regex::{Captures, Regex};

fn match_u32(caps: &Captures, i: usize) -> u32 {
    caps.get(i).and_then(|m| m.as_str().parse::<u32>().ok()).unwrap()
}

fn get_rules(lines: &Vec<String>) -> Vec<(u32, u32)> {
    let rules_re = Regex::new(r"(\d+)-(\d+)").unwrap();
    let mut rules = Vec::new();

    for line in lines {
        if line == "" {
            break;
        }

        for caps in rules_re.captures_iter(line) {
            let low = match_u32(&caps, 1);
            let high = match_u32(&caps, 2);
            rules.push((low, high));
        }
    }

    rules
}

fn invalid(rules: &Vec<(u32, u32)>, n: u32) -> Option<u32> {
    for &(low, high) in rules {
        if n >= low && n <= high {
            return None;
        }
    }

    Some(n)
}

pub fn part1(lines: &Vec<String>) -> u32 {
    let rules = get_rules(lines);
    let tickets = lines.iter()
                       .skip_while(|&line| !line.starts_with("nearby"))
                       .skip(1);

    let res: u32 = tickets.flat_map(|t| t.split(',')
        .filter_map(|s| {
            s.parse::<u32>().ok()
             .and_then(|n| invalid(&rules, n))
        })).sum();

    println!("{}", res);
    res
}
