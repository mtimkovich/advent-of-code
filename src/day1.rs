use std::collections::HashMap;
use std::collections::HashSet;

pub fn run(expense: &Vec<u32>) -> Option<u32> {
    let mut diffs = HashSet::new();

    for n in expense {
        let diff = 2020 - n;
        if diffs.contains(&diff) {
            return Some(n * diff);
        }

        diffs.insert(*n);
        diffs.insert(diff);
    }

    None
}

pub fn part2(expense: &Vec<u32>) -> Option<u32> {
    let mut sums = HashMap::new();

    for (i, x) in expense.iter().enumerate() {
        for y in expense.iter().skip(i) {
            sums.insert(x + y, vec![x, y]);
        }
    }

    for n in expense {
        let diff = 2020 - n;

        match sums.get(&diff) {
            Some(v) => return Some(v[0] * v[1] * n),
            None => (),
        };
    }

    None
}
