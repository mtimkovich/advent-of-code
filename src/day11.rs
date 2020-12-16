use std::cmp::PartialEq;
use std::fmt;

#[derive(Clone)]
struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn parse(lines: &Vec<String>) -> Self {
        let mut grid: Vec<Vec<char>> = Vec::new();
        for line in lines {
            grid.push(line.chars().collect());
        }

        Grid{grid}
    }

    fn adjacent(&self, x: usize, y: usize, i: i32, j: i32) -> Option<&char> {
        let mut x: i32 = x as i32;
        let mut y: i32 = y as i32;
        x += i;
        y += j;

        if (i == 0 && j == 0) || (x < 0 || y < 0) {
            return None;
        }

        self.grid.get(x as usize).and_then(|g| g.get(y as usize))
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Neighbors {
        let mut neighbors = Neighbors{empty: 0, occupied: 0};

        for i in -1..2i32 {
            for j in -1..2i32 {
                if (i == 0 && j == 0) || (x == 0 && i == -1) || (y == 0 && j == -1) {
                    continue;
                }

                match self.adjacent(x, y, i, j) {
                    Some('L') => neighbors.empty += 1,
                    Some('#') => neighbors.occupied += 1,
                    _ => (),
                };
            }
        }

        neighbors
    }

    fn predict(&self) -> Grid {
        let mut new_grid = self.grid.clone();

        for (x, row) in self.grid.iter().enumerate() {
            for (y, &val) in row.iter().enumerate() {
                if val == '.' {
                    continue;
                }
                let neighbors = self.get_neighbors(x, y);

                if val == 'L' && neighbors.occupied == 0 {
                    new_grid[x][y] = '#';
                } else if val == '#' && neighbors.occupied >= 4 {
                    new_grid[x][y] = 'L';
                }
            }
        }

        Grid{grid: new_grid}
    }

    fn occupied(&self) -> usize {
        self.grid.iter().flat_map(|row| {
            row.iter().filter(|&&i| i == '#')
        }).count()
    }

    fn simulation(&self) -> Grid {
        let mut cur_grid = self.clone();
        let mut prev_grid = self.clone();
        for i in 0.. {
            cur_grid = prev_grid.predict();
            if cur_grid == prev_grid {
                println!("{} iterations", i);
                return cur_grid;
            }

            prev_grid = cur_grid.clone();
        }

        cur_grid
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        for row in self.grid.iter() {
            for x in row {
                write!(f, "{}", x).ok();
            }
            write!(f, "\n").ok();
        }

        Ok(())
    }
}

impl PartialEq for Grid {
    fn eq(&self, other: &Self) -> bool {
        self.grid == other.grid
    }
}

#[derive(Debug)]
struct Neighbors {
    empty: u32,
    occupied: u32,
}

pub fn part1(lines: &Vec<String>) -> usize {
    let grid = Grid::parse(&lines);
    // let prediction = grid.predict();
    let prediction = grid.simulation();

    println!("{} seats occupied", prediction.occupied());
    // println!("{}", prediction);

    prediction.occupied()
}
