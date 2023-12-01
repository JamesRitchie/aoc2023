use std::{path::PathBuf, fs};

const NUMBER_REPLACEMENT: [(&str, &str); 9] = [
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9")
];

fn replace_words(original_line: &str, reverse: bool) -> String {
    let mut new_line = String::new();

    if reverse {
        for c in original_line.chars().rev() {
            new_line = c.to_string() + &new_line;
            for (target, replacement) in NUMBER_REPLACEMENT {
                new_line = new_line.replace(target, replacement);
            }
        }

    } else {

        for c in original_line.chars() {
            new_line.push(c);
            for (target, replacement) in NUMBER_REPLACEMENT {
                new_line = new_line.replace(target, replacement);
            }
        }
    };
    new_line
}

fn parse_line(line: &str, part_two: bool) -> i32 {

    let first_digit;
    let last_digit;
    if part_two {
        let forward_line = replace_words(&line, false);
        let forward_digits: Vec<char> = forward_line.chars().filter(|c| c.is_digit(10)).collect();
        first_digit = forward_digits.first().unwrap().clone();

        let reverse_line = replace_words(&line, true);
        let reverse_digits: Vec<char> = reverse_line.chars().filter(|c| c.is_digit(10)).collect();
        last_digit = reverse_digits.last().unwrap().clone();
    } else {
        let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
        first_digit = digits.first().unwrap_or(&'0').clone();
        last_digit = digits.last().unwrap_or(&'0').clone();
    }

    [first_digit, last_digit]
        .iter()
        .collect::<String>()
        .parse()
        .unwrap()
}

fn compute_answer(puzzle_input: &str, part_two: bool) -> i32 {
    puzzle_input
        .lines()
        .map(|l| parse_line(l, part_two))
        .sum()
}

pub fn run(input_path: PathBuf, part_two: bool) -> i32 {
    let puzzle_input = fs::read_to_string(input_path).unwrap();
    let answer = compute_answer(&puzzle_input, part_two);
    println!("The answer is {answer}");
    answer
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_line_part_one() {
        let line_value = parse_line("a1b2c3d4e5f", false);
        assert_eq!(line_value, 15);
    }

    #[test]
    fn test_parse_line_part_two() {
        let line_value = parse_line("7pqrstsixteen", true);
        assert_eq!(line_value, 76);
    }

    #[test]
    fn test_compute_answer_part_one() {
        let lines = "pqr3stu8vwx\na1b2c3d4e5f\n";
        let answer = compute_answer(lines, false);
        assert_eq!(answer, 53)
    }

    #[test]
    fn test_compute_answer_part_two() {
        let lines = "two1nine\neightwothree\n";
        let answer = compute_answer(lines, true);
        assert_eq!(answer, 112);
    }


}