use std::{collections::HashMap, error::Error, fs, path::PathBuf};

use num::integer::lcm;

fn compute_answer(puzzle_input: &str, part_two: bool) -> i64 {
    let (instruction_str, nodes) = puzzle_input.split_once("\n\n").unwrap();

    let mut node_map: HashMap<&str, (&str, &str)> = HashMap::new();

    // Using a string-based node lookup to minimise writing.
    for n in nodes.lines() {
        node_map.insert(&n[..3], (&n[7..10], &n[12..15]));
    }

    let starting_nodes;

    if part_two {
        starting_nodes = node_map
            .keys()
            .filter(|k| k.chars().nth(2).unwrap() == 'A')
            .map(|k| *k)
            .collect::<Vec<_>>()
    } else {
        starting_nodes = vec!["AAA"];
    }

    let mut step_counts: Vec<i64> = vec![];

    for n in starting_nodes.into_iter() {
        let mut current_node = n;
        let mut steps = 0;
        for instruction in instruction_str.chars().cycle() {
            current_node = match instruction {
                'L' => node_map[current_node].0,
                'R' => node_map[current_node].1,
                _ => panic!("Invalid instruction!"),
            };
            steps += 1;
            if part_two {
                if current_node.chars().nth(2).unwrap() == 'Z' {
                    step_counts.push(steps);
                    break;
                }
            } else {
                if current_node == "ZZZ" {
                    step_counts.push(steps);
                    break;
                }
            }
        }
    }

    // LCM will find the step where each cycle coincides at the end nodes.
    step_counts.iter().fold(1, |a, b| lcm(a, *b))
}

pub fn run(input_path: PathBuf, part_two: bool) -> Result<i64, Box<dyn Error>> {
    let puzzle_input = fs::read_to_string(input_path)?;
    let answer = compute_answer(&puzzle_input, part_two);
    println!("The answer is {answer}");
    Ok(answer)
}
