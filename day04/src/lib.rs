use std::{collections::HashSet, error::Error, fs, path::PathBuf};

fn parse_line(line: &str) -> usize {
    let (left, right) = line.split_once(" | ").unwrap();

    let left_numbers = left
        .split(": ")
        .last()
        .unwrap()
        .split(" ")
        .filter(|s| s.len() > 0);

    let right_numbers = right.split(" ").filter(|s| s.len() > 0);

    let left_set = HashSet::<_>::from_iter(left_numbers);
    let right_set = HashSet::<_>::from_iter(right_numbers);

    left_set.intersection(&right_set).count()
}

fn compute_answer(puzzle_input: &str, part_two: bool) -> i32 {
    let card_matches = puzzle_input.lines().map(|l| parse_line(l));

    if part_two {
        let card_values = card_matches.collect::<Vec<_>>();
        let mut card_counts = vec![1; card_values.len()];

        for (i, cv) in card_values.iter().enumerate() {
            for j in (i + 1)..(i + 1 + cv) {
                card_counts[j] += card_counts[i];
            }
        }

        card_counts.into_iter().sum()
    } else {
        card_matches
            .map(|c| {
                if c == 0 {
                    0
                } else {
                    i32::pow(2, (c - 1) as u32)
                }
            })
            .sum()
    }
}

pub fn run(input_path: PathBuf, part_two: bool) -> Result<i32, Box<dyn Error>> {
    let puzzle_input = fs::read_to_string(input_path)?;
    let answer = compute_answer(&puzzle_input, part_two);
    println!("The answer is {answer}");
    Ok(answer)
}
