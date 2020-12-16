use regex::Regex;

fn parse_buses(line: &String) -> Vec<u32> {
    let re = Regex::new(r"(\d+)").unwrap();
    let mut buses = Vec::new();
    for caps in re.captures_iter(line) {
        buses.push(caps.get(1).and_then(|s| s.as_str().parse::<u32>().ok()).unwrap());
    }

    buses
}

pub fn part1(lines: &Vec<String>) {
    let timestamp = lines[0].parse::<u32>().unwrap();
    let buses = parse_buses(&lines[1]);

    let res = buses.iter().map(|b| {
        let quotient = timestamp / b;
        let mut next_bus = b * quotient;
        if next_bus < timestamp {
            next_bus += b;
        }
        (next_bus - timestamp, *b)
    }).min().unwrap();

    println!("{:?}", res.0 * res.1);
}
