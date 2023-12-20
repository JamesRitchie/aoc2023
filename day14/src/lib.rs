use std::{collections::HashMap, error::Error, fs, path::PathBuf};

use util::transpose;

enum TiltDirection {
    North,
    East,
    South,
    West,
}

fn flip(rocks: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    rocks
        .iter()
        .map(|l| {
            let mut l_reversed = l.clone();
            l_reversed.reverse();
            l_reversed
        })
        .collect()
}

fn tilt_rocks(rocks: &Vec<Vec<char>>, direction: &TiltDirection) -> Vec<Vec<char>> {
    let mut rocks_tilted = match direction {
        TiltDirection::North => flip(&transpose(&rocks)),
        TiltDirection::East => rocks.clone(),
        TiltDirection::South => transpose(&rocks),
        TiltDirection::West => flip(&rocks.clone()),
    };

    rocks_tilted = rocks_tilted
        .iter()
        .map(|column| {
            column
                .split(|c| c == &'#')
                .map(|col| {
                    let mut col_sorted = col.to_vec();
                    col_sorted.sort();
                    col_sorted
                })
                .collect::<Vec<_>>()
                .join(&'#')
        })
        .collect::<Vec<_>>();

    match direction {
        TiltDirection::North => transpose(&flip(&rocks_tilted)),
        TiltDirection::East => rocks_tilted,
        TiltDirection::South => transpose(&rocks_tilted),
        TiltDirection::West => flip(&rocks_tilted),
    }
}

fn spin_rocks(rocks: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let spin_cycle = [
        TiltDirection::North,
        TiltDirection::West,
        TiltDirection::South,
        TiltDirection::East,
    ];

    spin_cycle.iter().fold(rocks, |r, c| tilt_rocks(&r, c))
}

fn compute_answer(puzzle_input: &str, part_two: bool) -> i64 {
    let rocks = puzzle_input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let n_rows = rocks.len();

    let mut rocks_tilted: Vec<Vec<char>>;

    if part_two {
        let mut state_history = HashMap::new();
        state_history.insert(rocks.clone(), 0);

        let mut cycle_detected = false;
        let mut i = 0;
        let mut j = 0;
        rocks_tilted = rocks;

        while !cycle_detected {
            rocks_tilted = spin_rocks(rocks_tilted);
            i += 1;

            if state_history.contains_key(&rocks_tilted) {
                cycle_detected = true;
                j = *state_history.get(&rocks_tilted).unwrap();
            } else {
                state_history.insert(rocks_tilted.clone(), i);
            }
        }

        let remaining_steps = (1000000000 - j) % (i - j);

        for _ in 0..remaining_steps {
            rocks_tilted = spin_rocks(rocks_tilted);
        }
    } else {
        rocks_tilted = tilt_rocks(&rocks, &TiltDirection::North);
    }

    transpose(&rocks_tilted)
        .iter()
        .map(|column| {
            column
                .iter()
                .enumerate()
                .filter(|(_, r)| **r == 'O')
                .map(|(i, _)| (n_rows - i) as i64)
                .sum::<i64>()
        })
        .sum()
}

pub fn run(input_path: PathBuf, part_two: bool) -> Result<i64, Box<dyn Error>> {
    let puzzle_input = fs::read_to_string(input_path)?;
    let answer = compute_answer(&puzzle_input, part_two);
    println!("The answer is {answer}");
    Ok(answer)
}
