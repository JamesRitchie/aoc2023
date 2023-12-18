use cached::proc_macro::cached;
use std::{error::Error, fs, path::PathBuf};

#[cached]
fn solve(springs: Vec<char>, current_group: i64, remaining_groups: Vec<i64>) -> i64 {
    let next_spring = springs[0];

    if next_spring == '?' {
        // Branch
        let mut s_hash = vec!['#'];
        s_hash.append(&mut springs[1..].to_vec().clone());
        let case_hash = solve(s_hash, current_group, remaining_groups.clone());

        let mut s_dot = vec!['.'];
        s_dot.append(&mut springs[1..].to_vec().clone());
        let case_dot = solve(s_dot, current_group, remaining_groups);

        return case_dot + case_hash;
    } else {
        if springs.len() == 1 {
            // Base case
            if remaining_groups.len() == 0 {
                match next_spring {
                    '.' if current_group == 0 => 1,
                    '#' if current_group == 1 => 1,
                    _ => 0,
                }
            } else if remaining_groups.len() == 1 && current_group == 0 {
                solve(springs, remaining_groups[0], remaining_groups[1..].to_vec())
            } else {
                0
            }
        } else {
            if current_group == 0 {
                match next_spring {
                    '.' => solve(springs[1..].to_vec(), 0, remaining_groups),
                    '#' if remaining_groups.len() == 0 => 0,
                    '#' if remaining_groups.len() > 0 => {
                        // Start a new group if possible
                        solve(springs, remaining_groups[0], remaining_groups[1..].to_vec())
                    }
                    _ => 0,
                }
            } else if current_group == 1 {
                // Current group must be closed.
                let second_spring = springs[1];
                match (next_spring, second_spring) {
                    ('#', '.') => solve(springs[1..].to_vec(), 0, remaining_groups),
                    ('#', '?') => {
                        let mut s = vec!['.'];
                        s.append(&mut springs[2..].to_vec().clone());
                        solve(s, 0, remaining_groups)
                    }
                    _ => 0,
                }
            } else {
                match next_spring {
                    // Current group must be extended.
                    '#' => solve(springs[1..].to_vec(), current_group - 1, remaining_groups),
                    _ => 0,
                }
            }
        }
    }
}

fn parse_line(line: &str, part_two: bool) -> i64 {
    let (spring_str, count_str) = line.split_once(" ").unwrap();
    let mut counts = count_str
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut springs = spring_str.chars().collect::<Vec<_>>();

    if part_two {
        springs = vec![springs; 5].join(&'?');
        counts = vec![counts; 5].concat();
    }

    solve(springs, 0, counts) as i64
}
fn compute_answer(puzzle_input: &str, part_two: bool) -> i64 {
    puzzle_input.lines().map(|l| parse_line(l, part_two)).sum()
}

pub fn run(input_path: PathBuf, part_two: bool) -> Result<i64, Box<dyn Error>> {
    let puzzle_input = fs::read_to_string(input_path)?;
    let answer = compute_answer(&puzzle_input, part_two);
    println!("The answer is {answer}");
    Ok(answer)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_case_one() {
        let springs = vec!['?', '?', '?', '.', '#', '#', '#'];
        let counts = vec![1, 1, 3];
        let answer = solve(springs, 0, counts);
        assert_eq!(answer, 1)
    }

    #[test]
    fn test_case_two() {
        let springs = vec![
            '.', '?', '?', '.', '.', '?', '?', '.', '.', '.', '?', '#', '#',
        ];
        let counts = vec![1, 1, 3];
        let answer = solve(springs, 0, counts);
        assert_eq!(answer, 4)
    }

    #[test]
    fn test_case_three() {
        let springs = vec!['?', '#', '#', '#', '?', '?', '?', '?', '?', '?', '?', '?'];
        let counts = vec![3, 2, 1];
        let answer = solve(springs, 0, counts);
        assert_eq!(answer, 10)
    }
}
