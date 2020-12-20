use itertools::iproduct;

fn usize_add(a: usize, b: i32) -> Option<usize> {
    if b >= 0 {
        return Some(a + b as usize);
    } else {
        let r = (a as i32) - b;
        if r < 0 {
            return None;
        } else {
            return Some(r as usize);
        }
    }
}

struct Grid(Vec<Vec<Vec<bool>>>);

impl Grid {
    fn new(lines: &Vec<String>) -> Self {
        let mut grid = vec![vec![vec![false; 3]; 3]; 3];
        for (x, line) in lines.iter().enumerate() {
            for (y, c) in line.chars().enumerate() {
                grid[x][y][0] = c == '#';
            }
        }

        Grid(grid)
    }

    fn neighbors(&self, x: usize, y: usize, z: usize) -> usize {
        let mut active = 0;
        for c in &[-1, 1] {
            if let Some(dx) = usize_add(x, *c) {
                let v = self.0.get(dx).and_then(|a| a.get(y)).and_then(|a| a.get(z));
                match v {
                    Some(true) => active += 1,
                    _ => (),
                };
            }
            if let Some(dy) = usize_add(y, *c) {
                let v = self.0.get(x).and_then(|a| a.get(dy)).and_then(|a| a.get(z));
                match v {
                    Some(true) => active += 1,
                    _ => (),
                };
            }
            if let Some(dz) = usize_add(z, *c) {
                let v = self.0.get(x).and_then(|a| a.get(y)).and_then(|a| a.get(dz));
                match v {
                    Some(true) => active += 1,
                    _ => (),
                };
            }
        }

        active
    }

    fn cycle(&self) -> Grid {
        let mut new_grid = self.0.clone();
        let len = self.0.len();
        for (x, y, z) in iproduct!(0..len, 0..len, 0..len) {
            let active = self.0[x][y][z];
            let space = &mut new_grid[x][y][z];

            let neigh = self.neighbors(x, y, z);
            println!("{}", neigh);
            if active && [2, 3].contains(&neigh) {
                *space = true;
            } else if !active && neigh == 3 {
                *space = true;
            } else {
                *space = false;
            }
        }

        Grid(new_grid)
    }

    fn slice(&self, z: usize) {
        for x in self.0.iter() {
            for y in x {
                if y[z] {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}

pub fn part1(lines: &Vec<String>) {
    let grid = Grid::new(lines);
    grid.slice(0);
    let new_grid = grid.cycle();
    new_grid.slice(0);
    new_grid.slice(1);
    // let a = grid.neighbors(0, 0, 0);
}
