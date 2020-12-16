use regex::Regex;
use std::collections::HashSet;

struct Proc {
    line: i32,
    acc: i32,
    visited: HashSet<i32>,
}

impl Proc {
    fn new() -> Self {
        Proc{
            line: 0,
            acc: 0,
            visited: HashSet::new(),
        }
    }

    fn exec(&mut self, cmd: &str, value: i32) {
        match cmd {
            "acc" => {
                self.acc += value;
                self.line += 1;
            },
            "jmp" => self.line += value,
            "nop" => self.line += 1,
            _ => panic!("invalid cmd"),
        };
    }

    fn run(&mut self, program: &Vec<String>) -> i32{
        let re = Regex::new(r"([a-z]+) (.*)").unwrap();

        while self.line < program.len() as i32 {
            let cap = re.captures(&program[self.line as usize]).unwrap();
            let cmd = cap.get(1).unwrap().as_str();
            let value = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();

            let good = self.visited.insert(self.line);

            if !good {
                return self.acc;
            }

            self.exec(cmd, value);
        }

        self.acc
    }
}

pub fn part1(program: &Vec<String>) {
    let mut proc = Proc::new();
    let fin = proc.run(program);
    println!("{}", fin);
}
