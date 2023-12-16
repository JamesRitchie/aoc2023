use itertools::Itertools;
use std::{error::Error, fs, path::PathBuf};

fn compute_answer(puzzle_input: &str, part_two: bool) -> i64 {
    let expansion_factor;
    if part_two {
        expansion_factor = 1000000i64
    } else {
        expansion_factor = 2i64;
    }

    let image = puzzle_input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let row_expansion_indices = image
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|c| *c == '.'))
        .map(|(row_index, _)| row_index as i64)
        .collect::<Vec<_>>();

    let col_expansion_indices = (0..image[0].len())
        .filter(|col_index| image.iter().map(|row| &row[*col_index]).all(|c| *c == '.'))
        .map(|col_index| col_index as i64)
        .collect::<Vec<_>>();

    let galaxies = image.iter().enumerate().flat_map(|(row_index, row)| {
        row.iter()
            .enumerate()
            .filter(|(_, g)| **g == '#')
            .map(move |(col_index, _)| (row_index as i64, col_index as i64))
    });

    galaxies
        .combinations(2)
        .map(|g| {
            // Find the number of intervening rows/columns to be expanded between each pair.
            let row_min = g[0].0.min(g[1].0);
            let row_max = g[0].0.max(g[1].0);
            let col_min = g[0].1.min(g[1].1);
            let col_max = g[0].1.max(g[1].1);

            let row_dist = (row_max - row_min)
                + row_expansion_indices
                    .iter()
                    .filter(|r| **r > row_min && **r < row_max)
                    .count() as i64
                    * (expansion_factor - 1);
            let col_dist = (col_max - col_min)
                + col_expansion_indices
                    .iter()
                    .filter(|c| **c > col_min && **c < col_max)
                    .count() as i64
                    * (expansion_factor - 1);
            row_dist + col_dist
        })
        .sum()
}

pub fn run(input_path: PathBuf, part_two: bool) -> Result<i64, Box<dyn Error>> {
    let puzzle_input = fs::read_to_string(input_path)?;
    let answer = compute_answer(&puzzle_input, part_two);
    println!("The answer is {answer}");
    Ok(answer)
}
