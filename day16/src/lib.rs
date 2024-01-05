use std::{collections::HashSet, error::Error, fs, path::PathBuf, str::FromStr};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Tile {
    Empty,
    LMirror,
    RMirror,
    VSplit,
    HSplit,
}

impl FromStr for Tile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Tile::Empty),
            "\\" => Ok(Tile::LMirror),
            "/" => Ok(Tile::RMirror),
            "|" => Ok(Tile::VSplit),
            "-" => Ok(Tile::HSplit),
            _ => Err(()),
        }
    }
}

struct Layout {
    grid: Vec<Vec<Tile>>,
    n_rows: usize,
    n_cols: usize,
    visited: HashSet<(usize, usize, Direction)>,
}

impl FromStr for Layout {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| Tile::from_str(&c.to_string()).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let n_rows = grid.len();
        let n_cols = grid[0].len();

        Ok(Layout {
            grid: grid,
            n_rows: n_rows,
            n_cols: n_cols,
            visited: HashSet::new(),
        })
    }
}

impl Layout {
    fn solve(&mut self, dir: Direction, r: i32, c: i32) {
        // Check that we're in the bounds of the grid and we haven't already visited this tile
        // while moving in the same direction.
        if r >= 0
            && c >= 0
            && r < self.n_rows as i32
            && c < self.n_cols as i32
            && !self.visited.contains(&(r as usize, c as usize, dir))
        {
            self.visited.insert((r as usize, c as usize, dir));
            let element = self.grid[r as usize][c as usize];
            let new_directions = match dir {
                Direction::Up => match element {
                    Tile::Empty | Tile::VSplit => vec![Direction::Up],
                    Tile::RMirror => vec![Direction::Right],
                    Tile::LMirror => vec![Direction::Left],
                    Tile::HSplit => vec![Direction::Right, Direction::Left],
                },
                Direction::Down => match element {
                    Tile::Empty | Tile::VSplit => vec![Direction::Down],
                    Tile::RMirror => vec![Direction::Left],
                    Tile::LMirror => vec![Direction::Right],
                    Tile::HSplit => vec![Direction::Left, Direction::Right],
                },
                Direction::Right => match element {
                    Tile::Empty | Tile::HSplit => vec![Direction::Right],
                    Tile::RMirror => vec![Direction::Up],
                    Tile::LMirror => vec![Direction::Down],
                    Tile::VSplit => vec![Direction::Up, Direction::Down],
                },
                Direction::Left => match element {
                    Tile::Empty | Tile::HSplit => vec![Direction::Left],
                    Tile::RMirror => vec![Direction::Down],
                    Tile::LMirror => vec![Direction::Up],
                    Tile::VSplit => vec![Direction::Up, Direction::Down],
                },
            };

            for new_dir in new_directions.into_iter() {
                match new_dir {
                    Direction::Up => self.solve(new_dir, r - 1, c),
                    Direction::Down => self.solve(new_dir, r + 1, c),
                    Direction::Right => self.solve(new_dir, r, c + 1),
                    Direction::Left => self.solve(new_dir, r, c - 1),
                }
            }
        }
    }

    fn start(&mut self, dir: Direction, r: i32, c: i32) -> usize {
        self.visited.clear();
        self.solve(dir, r, c);
        let energised_tiles: HashSet<(usize, usize)> =
            HashSet::from_iter(self.visited.iter().map(|(r, c, _)| (*r, *c)));
        energised_tiles.len()
    }
}

fn compute_answer(puzzle_input: &str, part_two: bool) -> i64 {
    let mut layout = Layout::from_str(&puzzle_input).unwrap();
    if part_two {
        // There is probably a more elegant way of doing this other than resetting at every start
        // point by caching results for each tile and direction, but the presence of cycles
        // complicates terminating the recursion, and it runs fast enough already.
        let top_edge = (0..layout.n_cols)
            .map(|i| layout.start(Direction::Down, 0, i as i32))
            .max()
            .unwrap() as i64;
        let bottom_edge = (0..layout.n_cols)
            .map(|i| layout.start(Direction::Up, (layout.n_rows - 1) as i32, i as i32))
            .max()
            .unwrap() as i64;
        let left_edge = (0..layout.n_rows)
            .map(|i| layout.start(Direction::Right, i as i32, 0 as i32))
            .max()
            .unwrap() as i64;
        let right_edge = (0..layout.n_rows)
            .map(|i| layout.start(Direction::Left, i as i32, (layout.n_cols - 1) as i32))
            .max()
            .unwrap() as i64;

        [top_edge, bottom_edge, left_edge, right_edge]
            .into_iter()
            .max()
            .unwrap()
    } else {
        layout.start(Direction::Right, 0, 0) as i64
    }
}

pub fn run(input_path: PathBuf, part_two: bool) -> Result<i64, Box<dyn Error>> {
    let puzzle_input = fs::read_to_string(input_path)?;
    let answer = compute_answer(&puzzle_input, part_two);
    println!("The answer is {answer}");
    Ok(answer)
}
