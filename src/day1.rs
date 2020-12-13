use std::collections::HashMap;
use std::collections::HashSet;

pub fn run(expense: &Vec<u32>) -> Result<u32, &'static str> {
    let mut diffs = HashSet::new();

    for n in expense {
        let diff = 2020 - n;
        if diffs.contains(&diff) {
            return Ok(n * diff);
        }

        diffs.insert(*n);
        diffs.insert(diff);
    }

    Err("No matching entries")
}

pub fn part2(expense: &Vec<u32>) -> Result<u32, &'static str> {
    let mut sums = HashMap::new();

    for (i, x) in expense.iter().enumerate() {
        for y in expense.iter().skip(i) {
            sums.insert(x + y, vec![x, y]);
        }
    }

    for n in expense {
        let diff = 2020 - n;
        if sums.contains_key(&diff) {
            let xs = sums.get(&diff).unwrap();
            return Ok(n * xs[0] * xs[1]);
        }
    }

    Err("No matching entries")
}
