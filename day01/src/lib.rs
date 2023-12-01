use std::{path::PathBuf, fs, error::Error};

// List of number words to convert to digits.
// Words could overlap, but by one char at most, so we include one char before and after the 
// replacement to avoid accidentally removing words. Some words like 'four' can't overlap, but we
// include the chars for consistency.
const NUMBER_REPLACEMENT: [(&str, &str); 9] = [
    ("one", "o1e"),
    ("two", "t2o"),
    ("three", "t3e"),
    ("four", "f4r"),
    ("five", "f5e"),
    ("six", "s6x"),
    ("seven", "s7n"),
    ("eight", "e8t"),
    ("nine", "n9e")
];

fn replace_number_strings(line: &str) -> String {
    let mut new_line = String::from(line);
    for (target, replacement) in NUMBER_REPLACEMENT {
        new_line = new_line.replace(target, replacement);
    };
    new_line
}

fn parse_line(line: &str, part_two: bool) -> i32 {

    let mut line = String::from(line);

    if part_two {
        line = replace_number_strings(&line);
    }

    let digits = line.chars().filter(|c| c.is_digit(10)).collect::<Vec<char>>();

    let first_digit = digits.first().unwrap_or(&'0');
    let last_digit = digits.last().unwrap_or(&'0');

    [first_digit, last_digit]
        .into_iter()
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

pub fn run(input_path: PathBuf, part_two: bool) -> Result<i32, Box<dyn Error>> {
    let puzzle_input = fs::read_to_string(input_path)?;
    let answer = compute_answer(&puzzle_input, part_two);
    println!("The answer is {answer}");
    Ok(answer)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_replace_number_strings() {
        let line = replace_number_strings("7pqrsthreeight");
        assert_eq!(line, "7pqrst3e8t")
    }

    #[test]
    fn test_parse_line_part_one() {
        let line_value = parse_line("a1b2c3d4e5f", false);
        assert_eq!(line_value, 15);
    }

    #[test]
    fn test_parse_line_part_two() {
        let line_value = parse_line("7pqrsthreeight", true);
        assert_eq!(line_value, 78);
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