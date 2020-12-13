use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;
mod day1;

fn read_lines(filename: &str) -> Result<Vec<String>, io::Error> {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines().collect::<Result<_, _>>()
}

fn main() {
    let filename = env::args().skip(1).next()
        .expect("filename is required");
    let path = format!("../inputs/{}", filename);
    let inputs = read_lines(&path).unwrap();

    let ints = inputs.into_iter()
                     .map(|s| s.parse::<u32>().unwrap())
                     .collect();
    let result = day1::part2(&ints);
    // let result = day1::run(&ints);
    match result {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{}", e),
    }
}
