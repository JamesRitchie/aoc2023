///
/// This is a pretty horrendous quicky and hacky first go and could do with a lot of tidying up.
use std::{error::Error, fs, path::PathBuf, str::FromStr};

use regex::{Match, Regex};

struct EngineSchematic {
    schematic_lines: Vec<String>,
    height: usize,
    width: usize,
}

impl FromStr for EngineSchematic {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let schematic_lines = s.lines().map(String::from).collect::<Vec<_>>();

        let height = schematic_lines.len();
        let width = schematic_lines[0].len();
        Ok(Self {
            schematic_lines: schematic_lines,
            height: height,
            width: width,
        })
    }
}

impl EngineSchematic {
    fn get_char(&self, i: i32, j: i32) -> char {
        if i < 0 || i >= self.height as i32 || j < 0 || j >= self.width as i32 {
            '.'
        } else {
            self.schematic_lines[i as usize]
                .chars()
                .nth(j as usize)
                .unwrap()
        }
    }

    fn get_line(&self, i: i32) -> &str {
        &self.schematic_lines[i as usize]
    }

    fn is_part(&self, i: i32, j: i32) -> bool {
        let c = self.get_char(i, j);
        (!c.is_digit(10)) && !(c == '.')
    }

    fn compute_part_sum(&self) -> i32 {
        let re = Regex::new(r"\d+").unwrap();

        let mut part_sum = 0;

        for (i, line) in self.schematic_lines.iter().enumerate() {
            let matches = re.find_iter(line);
            for m in matches {
                let start = m.start() as i32;
                let stop = m.end() as i32;

                let mut part_adjacent = false;

                for offset in [-1, 1] {
                    let r = i as i32 + offset;
                    for c in (start - 1)..(stop + 1) {
                        if self.is_part(r, c) {
                            part_adjacent = true;
                        }
                    }
                }

                if self.is_part(i as i32, start - 1) || self.is_part(i as i32, stop) {
                    part_adjacent = true;
                }

                if part_adjacent {
                    part_sum += m.as_str().parse::<i32>().unwrap();
                }
            }
        }
        part_sum
    }

    fn get_gear_neighbours(&self, i: usize, j: usize) -> i32 {
        let re = Regex::new(r"\d+").unwrap();

        let mut neighbours: Vec<i32> = vec![];

        let i = i as i32;
        let j = j as i32;

        for offset in [-1, 1] {
            if self.get_char(i + offset, j).is_digit(10) {
                let matches = re.find_iter(self.get_line(i + offset));
                neighbours.push(get_match_over_index(matches, j));
            } else {
                if self.get_char(i + offset, j - 1).is_digit(10) {
                    let matches = re.find_iter(self.get_line(i + offset));
                    neighbours.push(get_match_over_index(matches, j - 1))
                }
                if self.get_char(i + offset, j + 1).is_digit(10) {
                    let matches = re.find_iter(self.get_line(i + offset));
                    neighbours.push(get_match_over_index(matches, j + 1));
                }
            }
            if self.get_char(i, j + offset).is_digit(10) {
                let matches = re.find_iter(self.get_line(i));
                neighbours.push(get_match_over_index(matches, j + offset))
            }
        }

        if neighbours.len() == 2 {
            neighbours[0] * neighbours[1]
        } else {
            0
        }
    }

    fn compute_gear_ratio_sum(&self) -> i32 {
        let mut gear_ratio_sum = 0;
        for (i, line) in self.schematic_lines.iter().enumerate() {
            let gears = line
                .chars()
                .enumerate()
                .filter(|(j, c)| c == &'*')
                .map(|(j, c)| j);
            for j in gears {
                gear_ratio_sum += self.get_gear_neighbours(i, j);
            }
        }

        gear_ratio_sum
    }
}

fn get_match_over_index<'a>(matches: impl Iterator<Item = Match<'a>>, index: i32) -> i32 {
    let j = index as usize;
    matches
        .filter(|m| j >= m.start() && j < m.end())
        .next()
        .unwrap()
        .as_str()
        .parse::<i32>()
        .unwrap()
}

fn compute_answer(puzzle_input: &str, part_two: bool) -> i32 {
    let engine_schematic = EngineSchematic::from_str(puzzle_input).unwrap();
    if part_two {
        engine_schematic.compute_gear_ratio_sum()
    } else {
        engine_schematic.compute_part_sum()
    }
}

pub fn run(input_path: PathBuf, part_two: bool) -> Result<i32, Box<dyn Error>> {
    let puzzle_input = fs::read_to_string(input_path)?;
    let answer = compute_answer(&puzzle_input, part_two);
    println!("The answer is {answer}");
    Ok(answer)
}
