use std::{error::Error, fs, path::PathBuf};

fn recursive_diff(values: &Vec<i64>) -> i64 {
    let diff = values[..]
        .windows(2)
        .map(|v| v[1] - v[0])
        .collect::<Vec<_>>();

    if diff.iter().all(|v| *v == 0) {
        0
    } else {
        recursive_diff(&diff) + diff.last().unwrap()
    }
}

fn parse_line(line: &str, part_two: bool) -> i64 {
    let mut values = line
        .split(" ")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    if part_two {
        values.reverse();
    }

    values.last().unwrap() + recursive_diff(&values)
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
    fn test_recursive_diff_zero() {
        let answer = recursive_diff(&vec![1, 1, 1, 1, 1]);
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_recursive_diff_nonzero() {
        let answer = recursive_diff(&vec![10, 13, 16, 21, 30, 45]);
        assert_eq!(answer, 23);
    }

    #[test]
    fn test_parse_line_part_one() {
        let answer = parse_line("10 13 16 21 30 45", false);
        assert_eq!(answer, 68);
    }

    #[test]
    fn test_parse_line_part_two() {
        let answer = parse_line("10 13 16 21 30 45", true);
        assert_eq!(answer, 5);
    }
}
