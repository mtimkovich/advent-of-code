fn halve(range: &(u32, u32)) -> u32 {
    (range.1 - range.0 + 1) / 2
}

fn lower(range: &mut (u32, u32)) {
    range.1 -= halve(range);
}

fn upper(range: &mut (u32, u32)) {
    range.0 += halve(range);
}

fn seat_id(seat: &String) -> u32 {
    let mut row = (0, 127);
    let mut column = (0, 7);

    for c in seat.chars() {
        match c {
            'F' => lower(&mut row),
            'B' => upper(&mut row),
            'L' => lower(&mut column),
            'R' => upper(&mut column),
            _ => (),
        }
    }

    row.0 * 8 + column.0
}

fn all_ids<'a>(passes: &'a Vec<String>) -> impl Iterator<Item = u32> + 'a {
    passes.into_iter().map(|p| seat_id(p))
}

pub fn part1(passes: &Vec<String>) -> u32 {
    all_ids(passes).max().unwrap()
}

pub fn part2(passes: &Vec<String>) -> Option<u32> {
    let mut ids = all_ids(passes).collect::<Vec<u32>>();
    ids.sort();
    for (i, id) in ids.iter().enumerate() {
        if i == ids.len() - 1 {
            break;
        }

        if ids[i + 1] - id != 1 {
            return Some(id + 1);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::day5::*;

    #[test]
    fn lower_test() {
        let cases = [
            ((0, 127), 63),
            ((32, 63), 47),
            ((44, 47), 45),
            ((44, 45), 44),
            ((4, 7), 5),
        ];
        for (mut range, res) in &cases {
            lower(&mut range);
            assert_eq!(range.1, *res);
        }
    }

    #[test]
    fn upper_test() {
        let cases = [
            ((0, 63), 32),
            ((32, 47), 40),
            ((40, 47), 44),
            ((0, 7), 4),
            ((4, 5), 5),
        ];
        for (mut range, res) in &cases {
            upper(&mut range);
            assert_eq!(range.0, *res);
        }
    }
}
