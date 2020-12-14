struct Loc {
    x: usize,
    y: usize,
    dx: usize,
    dy: usize,
}

impl Loc {
    fn new(dx: usize, dy: usize) -> Self {
        Loc {x: 0, y: 0, dx, dy}
    }

    fn update(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }
}

fn count_trees(lines: &Vec<String>, loc: &mut Loc) -> u32 {
    let mut trees = 0;
    for (y, line) in lines.into_iter().enumerate() {
        if loc.y > y {
            continue;
        }
        let row: Vec<char> = line.chars().collect();
        let point = row[loc.x % row.len()];
        if point == '#' {
            trees += 1;
        }
        loc.update();
    }

    trees
}

pub fn part1(lines: &Vec<String>) -> u32 {
    let mut loc = Loc::new(3, 1);
    count_trees(&lines, &mut loc)
}

pub fn part2(lines: &Vec<String>) -> u32 {
    let mut product = 1;
    let locs = vec![
        Loc::new(1, 1),
        Loc::new(3, 1),
        Loc::new(5, 1),
        Loc::new(7, 1),
        Loc::new(1, 2),
    ];

    for mut loc in locs {
        let trees = count_trees(&lines, &mut loc);
        product *= trees;
    }

    product
}
