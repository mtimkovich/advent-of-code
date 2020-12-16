use regex::Regex;

fn parse_busses(line: &String) -> Vec<u32> {
    let re = Regex::new(r"(\d+)").unwrap();
    let mut busses = Vec::new();
    for caps in re.captures_iter(line) {
        busses.push(caps.get(1).and_then(|s| s.as_str().parse::<u32>().ok()).unwrap());
    }

    busses
}

pub fn part1(lines: &Vec<String>) {
    let timestamp = lines[0].parse::<u32>().unwrap();
    let busses = parse_busses(&lines[1]);

    let res = busses.iter().map(|b| {
        let quotient = timestamp / b;
        let mut next_bus = b * quotient;
        if next_bus < timestamp {
            next_bus += b;
        }
        (next_bus - timestamp, b.clone())
    }).min().unwrap();

    println!("{:?}", res.0 * res.1);
}
