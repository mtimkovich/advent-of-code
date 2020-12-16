use std::collections::VecDeque;

fn valid(n: u64, queue: &VecDeque<u64>) -> bool {
    for i in queue {
        for j in queue {
            if i + j == n {
                return true;
            }
        }
    }

    false
}

pub fn part1(inputs: &Vec<u64>) -> u64{
    let mut queue = VecDeque::new();
    let queue_size = 25;

    for &n in inputs {
        if queue.len() < queue_size {
            queue.push_back(n);
            continue;
        }

        if !valid(n, &queue) {
            return n;
        }

        queue.push_back(n);
        queue.pop_front();
    }

    0
}

pub fn part2(inputs: &Vec<u64>) {
    let target = part1(inputs);
    let mut queue = VecDeque::new();
    let mut sum = 0;
    let mut iter = inputs.iter();

    while sum != target {
        if queue.len() < 2 || sum < target {
            let n = *iter.next().unwrap();
            sum += n;
            queue.push_back(n);
        } else if sum > target {
            let first = queue.pop_front().unwrap();
            sum -= first;
        }
    }

    let min = queue.iter().min().unwrap();
    let max = queue.iter().max().unwrap();
    let weakness = min + max;

    println!("{}", weakness);
}
