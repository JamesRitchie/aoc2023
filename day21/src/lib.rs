// Day 21 of Advent of Code, 2023
use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    fs,
    path::PathBuf,
    str::FromStr,
};

#[derive(PartialEq, Clone)]
enum Tile {
    Garden,
    Rock,
    Start,
}

impl FromStr for Tile {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "." => Ok(Tile::Garden),
            "#" => Ok(Tile::Rock),
            "S" => Ok(Tile::Start),
            _ => Err(()),
        }
    }
}

fn parse_input(puzzle_input: &str) -> Vec<Vec<Tile>> {
    puzzle_input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Tile::from_str(&c.to_string()).unwrap())
                .collect()
        })
        .collect()
}

fn compute_answer(puzzle_input: &str, part_two: bool) -> i64 {
    let grid = parse_input(puzzle_input);

    let height = grid.len();
    let width = grid[0].len();

    // Find the starting position
    let start_pos = grid
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .position(|tile| *tile == Tile::Start)
                .map(|j| (i, j))
        })
        .unwrap();

    let mut visited = HashMap::new();
    let mut queue = VecDeque::from([(start_pos, 0)]);

    // BFS to find all reachable tiles and their steps
    while let Some(((x, y), steps)) = queue.pop_front() {
        if visited.contains_key(&(x, y)) {
            continue;
        }

        visited.insert((x, y), steps);

        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;

            if new_x < 0 || new_x >= height as i32 || new_y < 0 || new_y >= width as i32 {
                continue;
            }

            let new_pos = (new_x as usize, new_y as usize);

            match grid[new_pos.0][new_pos.1] {
                Tile::Garden | Tile::Start => {
                    if !visited.contains_key(&(new_pos)) {
                        queue.push_back((new_pos, steps + 1));
                    }
                }
                Tile::Rock => {}
            }
        }
    }

    if part_two {
        // This part of the solution entirely thanks to https://advent-of-code.xavd.id/writeups/2023/day/21/
        // and  https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
        let max_steps = 26501365;
        let edge_distance = (width / 2) as i64;
        let max_squares = (max_steps - edge_distance) / width as i64;

        let odd_squares = (max_squares + 1).pow(2);
        let even_squares = max_squares.pow(2);

        let odd_corners = visited
            .values()
            .filter(|&steps| steps > &edge_distance && steps % 2 == 1)
            .count() as i64;
        let even_corners = visited
            .values()
            .filter(|&steps| steps > &edge_distance && steps % 2 == 0)
            .count() as i64;

        let even_full = visited.values().filter(|&steps| steps % 2 == 0).count() as i64;
        let odd_full = visited.values().filter(|&steps| steps % 2 == 1).count() as i64;

        let answer = (odd_squares * odd_full) + (even_squares * even_full)
            - ((max_squares + 1) * odd_corners)
            + (max_squares * even_corners);
        answer as i64
    } else {
        let max_steps = 64;
        visited
            .values()
            .filter(|&steps| steps <= &max_steps && steps % 2 == 0)
            .count() as i64
    }
}

pub fn run(input_path: PathBuf, part_two: bool) -> Result<i64, Box<dyn Error>> {
    let puzzle_input = fs::read_to_string(input_path)?;
    let answer = compute_answer(&puzzle_input, part_two);
    println!("The answer is {answer}");
    Ok(answer)
}
