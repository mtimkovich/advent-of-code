fn setup(inputs: &Vec<u32>) -> Vec<u32> {
    let mut joltages = inputs.clone();
    joltages.push(0);
    joltages.sort();
    joltages.push(joltages.last().unwrap() + 3);
    joltages
}

pub fn part1(inputs: &Vec<u32>) {
    let joltages = setup(inputs);
    let mut ones = 0;
    let mut threes = 0;
    for (i, jolt) in joltages.iter().enumerate() {
        if i == joltages.len() - 1 {
            break;
        }

        match joltages[i + 1] - jolt {
            1 => ones += 1,
            3 => threes += 1,
            _ => println!("something else"),
        }
    }

    println!("1: {}, 3: {}", ones, threes);
    let res = ones * threes;
    println!("{}", res);
}

pub fn part2(inputs: &Vec<u32>) {
    let joltages = setup(inputs);
}
