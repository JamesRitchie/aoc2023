use std::{error::Error, fs, iter::zip, path::PathBuf};

fn find_vertical_reflection(block: &Vec<Vec<char>>, part_two: bool) -> i64 {
    for split in 1..block.len() {
        let mut upper = block[..split].to_vec();
        let mut lower = block[split..].to_vec();
        if lower.len() > upper.len() {
            lower = lower[..upper.len()].to_vec();
        } else if lower.len() < upper.len() {
            upper = upper[(upper.len() - lower.len())..].to_vec()
        }

        lower.reverse();

        if part_two {
            let diffs = zip(upper, lower)
                .flat_map(|(a, b)| zip(a, b).filter(|(x, y)| *x != *y))
                .count();
            if diffs == 1 {
                return split as i64;
            }
        } else {
            if upper == lower {
                return split as i64;
            }
        }
    }
    0
}

fn transpose<T>(block: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    (0..(block[0].len()))
        .map(|c| block.iter().map(|r| r[c].clone()).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn parse_block(block_str: &str, part_two: bool) -> i64 {
    let block = block_str
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let v_reflect = find_vertical_reflection(&block, part_two);
    let block_transpose = transpose(&block);
    let h_reflect = find_vertical_reflection(&block_transpose, part_two);

    h_reflect + (100 * v_reflect)
}

fn compute_answer(puzzle_input: &str, part_two: bool) -> i64 {
    puzzle_input
        .split("\n\n")
        .map(|b| parse_block(&b, part_two))
        .sum()
}

pub fn run(input_path: PathBuf, part_two: bool) -> Result<i64, Box<dyn Error>> {
    let puzzle_input = fs::read_to_string(input_path)?;
    let answer = compute_answer(&puzzle_input, part_two);
    println!("The answer is {answer}");
    Ok(answer)
}
