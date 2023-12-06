use std::{error::Error, fs, iter::zip, path::PathBuf};

fn count_ways(time: i64, distance: i64) -> i64 {
    // Solve the quadratic equation
    let a = -1.0;
    let b = time as f64;
    let c = -(distance) as f64;

    let sqrt = (b.powi(2) - 4.0 * a * c).sqrt();
    let root_1 = (-b - sqrt) / (2.0 * a);
    let root_2 = (-b + sqrt) / (2.0 * a);

    // Required to ensure we get the values of t that beat the distance.
    (root_1.ceil() - root_2.floor() - 1.0) as i64
}

fn compute_answer(puzzle_input: &str, part_two: bool) -> i64 {
    let (time_str, distance_str) = puzzle_input.split_once("\n").unwrap();

    if part_two {
        let time = time_str[5..].replace(" ", "").parse::<i64>().unwrap();
        let distance = distance_str[9..].replace(" ", "").parse::<i64>().unwrap();
        count_ways(time, distance)
    } else {
        let times = time_str[5..]
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap());

        let distances = distance_str[9..]
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap());

        zip(times, distances)
            .map(|(t, d)| count_ways(t, d))
            .product()
    }
}

pub fn run(input_path: PathBuf, part_two: bool) -> Result<i64, Box<dyn Error>> {
    let puzzle_input = fs::read_to_string(input_path)?;
    let answer = compute_answer(&puzzle_input, part_two);
    println!("The answer is {answer}");
    Ok(answer)
}
