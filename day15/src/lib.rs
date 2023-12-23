use std::{error::Error, fs, path::PathBuf};

struct LensBoxes {
    boxes: [Vec<(String, usize)>; 256],
}

impl LensBoxes {
    fn new() -> LensBoxes {
        LensBoxes {
            boxes: vec![Vec::new(); 256].try_into().unwrap(),
        }
    }

    fn parse_instruction(&mut self, instruction: &str) {
        if instruction.contains("=") {
            self.add_lens(instruction);
        } else {
            self.remove_lens(instruction)
        }
    }

    fn add_lens(&mut self, instruction: &str) {
        let (label, value_str) = instruction.split_once("=").unwrap();
        let label_index = parse_label(label);
        let value = value_str.parse::<usize>().unwrap();

        if let Some(lens) = self.boxes[label_index].iter_mut().find(|l| l.0 == label) {
            lens.1 = value;
        } else {
            self.boxes[label_index].push((label.to_string(), value));
        }
    }

    fn remove_lens(&mut self, instruction: &str) {
        let (label, _) = instruction.split_once("-").unwrap();
        let label_index = parse_label(label);
        self.boxes[label_index].retain(|(l, _)| l != label);
    }

    fn focusing_power(&self) -> i64 {
        let power: usize = self
            .boxes
            .iter()
            .enumerate()
            .map(|(i, b)| {
                b.iter()
                    .enumerate()
                    .map(|(j, l)| (i + 1) * (j + 1) * l.1)
                    .sum::<usize>()
            })
            .sum();
        power as i64
    }
}

fn parse_label(label: &str) -> usize {
    label
        .bytes()
        .fold(0, |a, c| ((a + (c as usize)) * 17) % 256)
}

fn compute_answer(puzzle_input: &str, part_two: bool) -> i64 {
    let instructions = puzzle_input.trim().split(",");

    if part_two {
        let mut lens_boxes = LensBoxes::new();
        for i in instructions {
            lens_boxes.parse_instruction(i);
        }
        lens_boxes.focusing_power()
    } else {
        let answer: usize = instructions.map(parse_label).sum();
        answer as i64
    }
}

pub fn run(input_path: PathBuf, part_two: bool) -> Result<i64, Box<dyn Error>> {
    let puzzle_input = fs::read_to_string(input_path)?;
    let answer = compute_answer(&puzzle_input, part_two);
    println!("The answer is {answer}");
    Ok(answer)
}
