use regex::Regex;
use std::collections::HashMap;

fn create_rules(lines: &Vec<String>) -> HashMap<String, Vec<String>> {
    let mut rules = HashMap::new();
    let re_parent = Regex::new(r"^(.*?) bags").unwrap();
    let re_children = Regex::new(r"\d (.*?) bag").unwrap();

    for line in lines {
        let cap = re_parent.captures(&line).unwrap();
        let parent = cap.get(1).unwrap().as_str().to_string();

        if !rules.contains_key(&parent) {
            rules.insert(parent.clone(), vec![]);
        }

        let children = rules.get_mut(&parent).unwrap();

        for caps in re_children.captures_iter(&line) {
            children.push(caps.get(1).unwrap().as_str().to_string());
        }
    }

    rules
}

fn search(start: &String, rules: &HashMap<String, Vec<String>>) -> Option<usize> {
    let goal = "shiny gold";
    let mut path = Vec::new();
    let mut stack = vec![start];
    let mut succ = false;

    while stack.len() > 0 {
        let node = stack.pop().unwrap();
        path.push(node);
        if node == goal {
            succ = true;
            break;
        }
        let children = rules.get(node).unwrap();
        for c in children {
            stack.push(c);
        }
    }

    if succ && path.len() > 1 {
        return Some(path.len());
    }

    None
}

pub fn part1(lines: &Vec<String>) -> usize {
    let rules = create_rules(lines);
    let res = rules.keys().filter_map(|k| search(&k, &rules)).count();

    println!("{}", res);
    res
}
