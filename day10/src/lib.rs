use std::{collections::HashSet, error::Error, fs, path::PathBuf, str::FromStr};

#[derive(PartialEq, Debug)]
enum TileType {
    Vertical,
    Horizontal,
    NorthEastBend,
    NorthWestBend,
    SouthWestBend,
    SouthEastBend,
    Ground,
    Start,
}

impl FromStr for TileType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "|" => Ok(Self::Vertical),
            "-" => Ok(Self::Horizontal),
            "L" => Ok(Self::NorthEastBend),
            "J" => Ok(Self::NorthWestBend),
            "7" => Ok(Self::SouthWestBend),
            "F" => Ok(Self::SouthEastBend),
            "." => Ok(Self::Ground),
            "S" => Ok(Self::Start),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Hash, Eq)]
struct Position {
    r: usize,
    c: usize,
}

impl TileType {
    fn get_offsets(&self) -> [(i32, i32); 2] {
        match self {
            Self::Vertical => [(-1, 0), (1, 0)],
            Self::Horizontal => [(0, -1), (0, 1)],
            Self::NorthEastBend => [(-1, 0), (0, 1)],
            Self::NorthWestBend => [(-1, 0), (0, -1)],
            Self::SouthWestBend => [(1, 0), (0, -1)],
            Self::SouthEastBend => [(1, 0), (0, 1)],
            _ => panic!("Invalid tile type for offsets."),
        }
    }
}

fn compute_answer(puzzle_input: &str, part_two: bool) -> i64 {
    let tile_map = puzzle_input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| TileType::from_str(&c.to_string()).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let n_rows = tile_map.len();
    let n_cols = tile_map[0].len();

    let starting_position = tile_map
        .iter()
        .enumerate()
        .find_map(|(row_index, row)| {
            row.iter()
                .position(|t| *t == TileType::Start)
                .map(|col_index| Position {
                    r: row_index,
                    c: col_index,
                })
        })
        .unwrap();

    let mut neighbouring_positions = vec![];

    let adjacent_offsets = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    for offset in adjacent_offsets {
        // Check each tile adjacent to the start to find the two tiles which must connect to it.

        // Subtract so we can compare the offsets produced by the neighbouring tile.
        let r = starting_position.r as i32 - offset.0;
        let c = starting_position.c as i32 - offset.1;

        if r >= 0 && r < n_rows as i32 && c >= 0 && c < n_cols as i32 {
            let t = &tile_map[r as usize][c as usize];
            match t {
                TileType::Ground => (),
                _ => {
                    if t.get_offsets().into_iter().any(|o| o == offset) {
                        // If the neighbouring tile has one of the same offsets it must connect
                        // to the start and therefore be part of the loop.
                        neighbouring_positions.push(Position {
                            r: r as usize,
                            c: c as usize,
                        });
                    }
                }
            }
        }
    }

    let mut last_position = starting_position;
    let mut current_position = neighbouring_positions[0];
    let mut step_count = 1;

    // Keep track of which tiles are part of the loop for use in part two.
    let mut loop_tiles = HashSet::new();
    loop_tiles.insert(last_position);
    loop_tiles.insert(current_position);

    while current_position != starting_position {
        // Follow the pipe sections around until we get back to the start.
        let t = &tile_map[current_position.r][current_position.c];
        let [offset_0, offset_1] = t.get_offsets();
        let position_0 = Position {
            r: (current_position.r as i32 + offset_0.0) as usize,
            c: (current_position.c as i32 + offset_0.1) as usize,
        };

        if position_0 == last_position {
            last_position = current_position;
            current_position = Position {
                r: (current_position.r as i32 + offset_1.0) as usize,
                c: (current_position.c as i32 + offset_1.1) as usize,
            }
        } else {
            last_position = current_position;
            current_position = position_0;
        }
        loop_tiles.insert(current_position);
        step_count += 1;
    }

    if part_two {
        // For each tile not part of the loop, count each time we cross a vertical section of the
        // loop when moving out to the left. If it's odd, it must be an interior tile.
        // Could be more efficient by doing a cumulative sum left to right.
        let mut interior_count = 0;
        for r in 0..n_rows {
            for c in 0..n_cols {
                if !loop_tiles.contains(&Position { r: r, c: c }) {
                    let mut n = 0;
                    for i in 0..c {
                        if loop_tiles.contains(&Position { r: r, c: i }) {
                            match tile_map[r][i] {
                                TileType::Vertical
                                | TileType::NorthEastBend
                                | TileType::NorthWestBend => n += 1,
                                _ => (),
                            }
                        }
                    }
                    if n % 2 == 1 {
                        interior_count += 1;
                    }
                }
            }
        }
        interior_count
    } else {
        if step_count % 2 == 0 {
            step_count / 2
        } else {
            (step_count / 2) + 1
        }
    }
}

pub fn run(input_path: PathBuf, part_two: bool) -> Result<i64, Box<dyn Error>> {
    let puzzle_input = fs::read_to_string(input_path)?;
    let answer = compute_answer(&puzzle_input, part_two);
    println!("The answer is {answer}");
    Ok(answer)
}
