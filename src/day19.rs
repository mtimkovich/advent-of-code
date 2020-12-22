use regex::Regex;
use std::collections::HashMap;

fn parse_rules(lines: &Vec<String>) -> (HashMap<u32, Vec<u32>>, HashMap<u32, char>) {
    let re = Regex::new(r"(\d+)").unwrap();
    let re_terminal = Regex::new(r"([ab])").unwrap();
    let mut rules = HashMap::new();
    let mut terminals = HashMap::new();

    for line in lines {
        if line == "" {
            break;
        }

        let caps: Vec<u32> = re
            .captures_iter(line)
            .filter_map(|m| m.get(1).unwrap().as_str().parse::<u32>().ok())
            .collect();

        if caps.len() == 1 {
            let c = re_terminal
                .captures(line)
                .and_then(|s| s.get(1).and_then(|s| s.as_str().chars().next()))
                .unwrap();
            terminals.insert(caps[0], c);
        } else {
            rules.insert(caps[0], caps[1..].to_vec());
        }
    }

    (rules, terminals)
}

pub fn part1(lines: &Vec<String>) {
    let (rules, terminals) = parse_rules(lines);
    println!("{:?}", rules);
    println!("{:?}", terminals);
}
