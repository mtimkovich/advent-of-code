#![allow(dead_code)]
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;

mod day8;

/// Read file into string vector.
fn read_lines(filename: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines().collect::<Result<_, _>>()
}

/// String vector to int vector.
fn to_ints(input: Vec<String>) -> Vec<u32> {
    input.into_iter()
          .map(|s| s.parse::<u32>().unwrap())
          .collect()
}

fn main() {
    let filename = env::args().skip(1).next()
        .expect("filename is required");
    let path = format!("../inputs/{}", filename);
    let inputs = read_lines(&path).unwrap();
    day8::part1(&inputs);
    // let result = day7::part1(&inputs);
    // println!("{}", result);
}
