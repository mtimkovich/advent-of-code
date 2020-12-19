use regex::{Captures, Regex};
use std::collections::HashMap;
use std::collections::HashSet;

fn match_u32(caps: &Captures, i: usize) -> u32 {
    caps.get(i)
        .and_then(|m| m.as_str().parse::<u32>().ok())
        .unwrap()
}

fn get_rule_names(lines: &Vec<String>) -> HashMap<&str, Vec<u32>> {
    let rules_re = Regex::new(r"(\d+)-(\d+)").unwrap();
    let mut rules = HashMap::new();

    for line in lines {
        if line == "" {
            break;
        }

        let mut split = line.split(':');
        let field = split.next().unwrap();
        let ranges = split.next().unwrap();

        for caps in rules_re.captures_iter(ranges) {
            let low = match_u32(&caps, 1);
            let high = match_u32(&caps, 2);

            if !rules.contains_key(&field) {
                rules.insert(field, Vec::new());
            }

            let v = rules.get_mut(field).unwrap();
            v.push(low);
            v.push(high);
        }
    }

    rules
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

fn get_tickets(lines: &Vec<String>) -> impl Iterator<Item = &String> {
    lines
        .iter()
        .skip_while(|&line| !line.starts_with("nearby"))
        .skip(1)
}

fn get_my_ticket(lines: &Vec<String>) -> Vec<u32> {
    get_values(
        lines
            .iter()
            .skip_while(|&line| !line.starts_with("your ticket"))
            .skip(1)
            .next()
            .unwrap(),
    )
    .collect()
}

fn get_values<'a>(ticket: &'a String) -> impl Iterator<Item = u32> + 'a {
    ticket.split(',').filter_map(|s| s.parse::<u32>().ok())
}

fn invalid(rules: &Vec<(u32, u32)>, n: u32) -> Option<u32> {
    for &(low, high) in rules {
        if n >= low && n <= high {
            return None;
        }
    }

    Some(n)
}

fn valid_for_field(rules: &Vec<u32>, n: u32) -> bool {
    (rules[0] <= n && n <= rules[1]) || (rules[2] <= n && n <= rules[3])
}

fn valid_for_any_field(n: u32, rules: &HashMap<&str, Vec<u32>>) -> bool {
    rules.iter().any(|(_, rule)| valid_for_field(rule, n))
}

fn valid_ticket(ticket: &String, rules: &HashMap<&str, Vec<u32>>) -> bool {
    get_values(ticket).all(|n| valid_for_any_field(n, rules))
}

fn valid_tickets<'a>(
    tickets: impl Iterator<Item = &'a String>,
    rules: &HashMap<&str, Vec<u32>>,
) -> Vec<&'a String> {
    let mut valid_tickets = Vec::new();
    for ticket in tickets {
        if valid_ticket(&ticket, rules) {
            valid_tickets.push(ticket);
        }
    }

    valid_tickets
}

fn field_indexes(possible_rules: HashMap<usize, HashSet<&str>>) -> HashMap<usize, &str> {
    let mut possibles: Vec<(usize, HashSet<&str>)> = possible_rules.into_iter().collect();
    possibles.sort_by(|a, b| a.1.len().cmp(&b.1.len()));

    let mut prev: HashSet<&str> = HashSet::new();

    let mut positions = HashMap::new();
    for (key, value) in possibles {
        let diff: Vec<&str> = value.difference(&prev).cloned().collect();
        prev.insert(diff[0]);
        positions.insert(key, diff[0]);
    }

    positions
}

pub fn part1(lines: &Vec<String>) -> u32 {
    let rules = get_rules(lines);
    let tickets = get_tickets(lines);

    let res = tickets
        .flat_map(|t| get_values(t).filter_map(|n| invalid(&rules, n)))
        .sum();

    println!("{}", res);
    res
}

pub fn part2(lines: &Vec<String>) {
    let rules = get_rule_names(lines);
    let my_ticket = get_my_ticket(lines);
    let all_tickets = get_tickets(lines);
    let tickets = valid_tickets(all_tickets, &rules);

    let mut possible_rules: HashMap<usize, HashSet<&str>> = HashMap::new();

    for (i, _) in rules.iter().enumerate() {
        let s: HashSet<&str> = rules.keys().cloned().collect();
        possible_rules.insert(i, s);
    }

    for ticket in tickets {
        for (i, n) in get_values(ticket).enumerate() {
            for (field, range) in rules.iter() {
                if !valid_for_field(range, n) {
                    possible_rules.get_mut(&i).unwrap().remove(field);
                }
            }
        }
    }

    let possibles = field_indexes(possible_rules);
    let res: u64 = possibles
        .into_iter()
        .filter(|(_, v)| v.contains("departure"))
        .map(|(i, _)| my_ticket[i] as u64)
        .product();
    println!("{}", res);
}
