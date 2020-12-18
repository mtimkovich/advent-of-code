use regex::{Captures, Regex};
use std::collections::HashMap;
use std::collections::HashSet;

fn match_u32(caps: &Captures, i: usize) -> u32 {
    caps.get(i).and_then(|m| m.as_str().parse::<u32>().ok()).unwrap()
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

<<<<<<< Updated upstream
pub fn part1(lines: &Vec<String>) {
    let rules = get_rules(lines);
=======
fn get_tickets(lines: &Vec<String>) -> impl Iterator<Item = &String> {
    lines
        .iter()
        .skip_while(|&line| !line.starts_with("nearby"))
        .skip(1)
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
>>>>>>> Stashed changes

    let mut tickets = lines.iter().skip_while(|&line| !line.starts_with("nearby")).skip(1);
    println!("{}", tickets.next().unwrap());

<<<<<<< Updated upstream
    println!("{:?}", rules);
=======
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

pub fn part1(lines: &Vec<String>) -> u32 {
    let rules = get_rules(lines);
    let tickets = get_tickets(lines);

    let res = tickets
        .flat_map(|t| get_values(t).filter_map(|n| invalid(&rules, n)))
        .sum();

    println!("{}", res);
    res
>>>>>>> Stashed changes
}

pub fn part2(lines: &Vec<String>) {
    let rules = get_rule_names(lines);
    let all_tickets = get_tickets(lines);
    let tickets = valid_tickets(all_tickets, &rules);
    println!("{}", tickets.len());

    let mut possible_rules: HashMap<usize, HashSet<&str>> = HashMap::new();

    for (i, _) in rules.iter().enumerate() {
        let s: HashSet<&str> = rules.keys().cloned().collect();
        possible_rules.insert(i, s);
    }

    for ticket in tickets {
        let values: Vec<u32> = get_values(ticket).collect();

        // For each index, have a hashset corresponding to each rule.
        // if a field doesn't match a rule, remove it from the set.
        // stop when each index only has 1.

        for (i, &n) in values.iter().enumerate() {
            for (field, range) in rules.iter() {
                if !valid_for_field(range, n) {
                    // println!("{}: {:?}", n, range);
                    // println!("invalid, removing {} from {}", field, i);
                    possible_rules.get_mut(&i).unwrap().remove(field);
                }
            }
        }
    }

    // TODO: Collect into vector and sort by value.len().
    for (key, value) in possible_rules.iter() {
        // if value.iter().find(|s| s.contains("departure")).is_none() {
        //     continue;
        // }

        println!("{}", key);
        println!("{}: {:?}", value.len(), value);
    }
}
