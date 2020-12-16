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

pub fn part1(inputs: &Vec<u64>) {
    let mut queue = VecDeque::new();
    let queue_size = 25;

    for n in inputs {
        if queue.len() < queue_size {
            queue.push_back(*n);
            continue;
        }

        if !valid(*n, &queue) {
            println!("{}", n);
            break;
        }

        queue.push_back(*n);
        queue.pop_front();
    }
}
