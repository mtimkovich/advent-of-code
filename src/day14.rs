use regex::{Captures, Regex};
use std::collections::HashMap;

fn match_u64(caps: &Captures, i: usize) -> u64 {
    caps.get(i)
        .and_then(|s| s.as_str().parse::<u64>().ok())
        .unwrap()
}

fn mask_on(mask: &str, value: u64) -> u64 {
    let mut value = value;
    for (n, bit) in mask.chars().rev().enumerate() {
        match bit {
            '0' => value &= !(1 << n),
            '1' => value |= 1 << n,
            _ => (),
        };
    }

    value
}

pub fn part1(lines: &Vec<String>) -> u64 {
    let mask_re = Regex::new(r"mask = ([X01]+)").unwrap();
    let mem_re = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
    let mut memory = HashMap::new();
    let mut mask = "";

    for line in lines {
        if line.starts_with("mask") {
            mask = mask_re
                .captures(line)
                .and_then(|c| c.get(1))
                .unwrap()
                .as_str();
        } else {
            let caps = mem_re.captures(line).unwrap();
            let addr = match_u64(&caps, 1);
            let value = match_u64(&caps, 2);

            let masked = mask_on(&mask, value);
            memory.insert(addr, masked);
        }
    }

    let res: u64 = memory.values().sum();
    println!("{}", res);
    res
}
