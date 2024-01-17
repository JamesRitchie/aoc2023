use std::{error::Error, fs, path::PathBuf};

fn compute_answer(puzzle_input: &str, part_two: bool) -> i64 {
    let instructions = puzzle_input.lines().map(|l| {
        let mut steps = l.split(" ");
        let direction;
        let count;
        if part_two {
            let hex_digits = steps.last().unwrap().chars().collect::<Vec<_>>();
            direction = match hex_digits[7] {
                '0' => "R",
                '1' => "D",
                '2' => "L",
                '3' => "U",
                _ => panic!(),
            };
            count = i64::from_str_radix(hex_digits[2..7].iter().collect::<String>().as_str(), 16)
                .unwrap();
        } else {
            direction = steps.next().unwrap();
            count = steps.next().unwrap().parse::<i64>().unwrap();
        }

        (direction, count)
    });

    // Vector of (x, y, boundary length)
    let mut coords_and_length = vec![(0, 0, 0)];

    // Can't do windows on iterators currently with Rust out-of-the-box, so need to collect it into
    // a vector.
    coords_and_length.append(
        &mut instructions
            .scan(coords_and_length[0], |state, (direction, count)| {
                match direction {
                    "U" => state.1 += count,
                    "D" => state.1 -= count,
                    "L" => state.0 -= count,
                    "R" => state.0 += count,
                    _ => panic!(),
                }
                state.2 += count;
                Some(*state)
            })
            .collect::<Vec<_>>(),
    );

    let boundary_length = coords_and_length.last().unwrap().2;

    // Shoelace formula to determine number of interior points.
    let interior_points = coords_and_length
        .windows(2)
        .map(|c| (c[0].0 * c[1].1) - (c[0].1 * c[1].0))
        .sum::<i64>()
        .abs() // Might be negative depending on the orientation of the sequence.
        / 2;

    // Modification of Pick's theorem to determine the total area.
    interior_points + (boundary_length / 2) + 1
}

pub fn run(input_path: PathBuf, part_two: bool) -> Result<i64, Box<dyn Error>> {
    let puzzle_input = fs::read_to_string(input_path)?;
    let answer = compute_answer(&puzzle_input, part_two);
    println!("The answer is {answer}");
    Ok(answer)
}
