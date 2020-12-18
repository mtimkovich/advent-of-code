use regex::Regex;

#[derive(Debug)]
struct Action {
    action: char,
    value: i32,
}

fn parse(lines: &Vec<String>) -> Vec<Action> {
    let re = Regex::new(r"([NSEWLRF])(\d+)").unwrap();
    let mut actions = Vec::new();
    for line in lines {
        let cap = re.captures(line).unwrap();
        let action = cap.get(1).unwrap().as_str().chars().next().unwrap();
        let value = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
        actions.push(Action { action, value });
    }

    actions
}

fn mul(a: i32, v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|x| a * x).collect()
}

fn add(a: &mut Vec<i32>, b: &Vec<i32>) {
    for (a, &b) in a.iter_mut().zip(b) {
        *a += b;
    }
}

fn manhattan(v: &Vec<i32>) -> i32 {
    v.iter().map(|i| i.abs()).sum()
}

pub fn part1(lines: &Vec<String>) {
    let actions = parse(lines);
    // +North, +East
    let mut loc = vec![0, 0];
    let mut face = 0;

    // This could probably be done better with math lmao
    let card = vec![vec![0, 1], vec![-1, 0], vec![0, -1], vec![1, 0]];

    for a in &actions {
        match a.action {
            'F' => {
                let mov = mul(a.value, &card[face as usize]);
                add(&mut loc, &mov);
            }
            'R' => {
                face = (face + a.value / 90) % card.len() as i32;
            }
            'L' => {
                face = (face - a.value / 90) % card.len() as i32;
                if face < 0 {
                    face += card.len() as i32;
                }
            }
            'N' => {
                add(&mut loc, &vec![a.value, 0]);
            }
            'S' => {
                add(&mut loc, &vec![-a.value, 0]);
            }
            'E' => {
                add(&mut loc, &vec![0, a.value]);
            }
            'W' => {
                add(&mut loc, &vec![0, -a.value]);
            }
            _ => (),
        }
    }

    println!("{}", manhattan(&loc));
}
