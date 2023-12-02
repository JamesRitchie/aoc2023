use std::{cmp, collections::HashMap, error::Error, fs, path::PathBuf, str::FromStr};

/// All the possible cube colours.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum CubeColour {
    Red,
    Green,
    Blue,
}

impl FromStr for CubeColour {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(CubeColour::Red),
            "green" => Ok(CubeColour::Green),
            "blue" => Ok(CubeColour::Blue),
            _ => Err(()),
        }
    }
}

/// Represents a draw of cubes, with number drawn of a particular colour.
struct CubeDraw {
    count: i32,
    colour: CubeColour,
}

impl FromStr for CubeDraw {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (count, colour) = s.split_once(" ").unwrap();
        let count = count.parse::<i32>().unwrap();

        Ok(CubeDraw {
            count: count,
            colour: CubeColour::from_str(colour)?,
        })
    }
}

impl CubeDraw {
    /// Check whether the draw is possible for part one given the total number of cubes for each
    /// colour in the bag provided by the puzzle description.
    fn invalid(&self) -> bool {
        let max_count = match self.colour {
            CubeColour::Red => 12,
            CubeColour::Green => 13,
            CubeColour::Blue => 14,
        };
        self.count > max_count
    }
}

/// Parse the line according to the rules in the puzzle.
/// For part one, return 0 if the game is not possible, otherwise return the game number.
/// For part two, return the product of the minimum possible number of each colour of cube.
fn parse_line(line: &str, part_two: bool) -> i32 {
    let (game, rounds) = line.split_once(": ").unwrap();

    // The separation of draws into rounds doesn't actually matter, so we can flatten them out
    // into one sequence.
    let mut draws = rounds
        .split("; ")
        .flat_map(|l| l.split(", "))
        .map(|d| CubeDraw::from_str(d).unwrap());

    if part_two {
        // Find the maximum seen count for each colour across all draws.
        let mut colour_maxes: HashMap<CubeColour, i32> = HashMap::new();
        colour_maxes = draws.fold(colour_maxes, |mut cm, draw| {
            let current = *cm.get(&draw.colour).unwrap_or(&0);
            let max = cmp::max(draw.count, current);
            cm.insert(draw.colour, max);
            cm
        });

        // let power = color_maxes.values().product();
        let power = colour_maxes.values().product();

        return power;
    } else {
        let invalid_game = draws.any(|draw| draw.invalid());

        if invalid_game {
            return 0;
        } else {
            return game[5..].parse::<i32>().unwrap();
        }
    }
}

fn compute_answer(puzzle_input: &str, part_two: bool) -> i32 {
    puzzle_input.lines().map(|l| parse_line(l, part_two)).sum()
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
    fn test_parse_line_part_one_valid() {
        let line = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let line_value = parse_line(line, false);
        assert_eq!(line_value, 5);
    }

    #[test]
    fn test_parse_line_part_one_invalid() {
        let line = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        let line_value = parse_line(line, false);
        assert_eq!(line_value, 0);
    }

    #[test]
    fn test_parse_line_part_two() {
        let line = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        let line_value = parse_line(line, true);
        assert_eq!(line_value, 630);
    }

    #[test]
    fn test_compute_answer_part_one() {
        let lines = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let answer = compute_answer(&lines, false);
        assert_eq!(answer, 5);
    }

    #[test]
    fn test_compute_answer_part_two() {
        let lines = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let answer = compute_answer(&lines, true);
        assert_eq!(answer, 666);
    }
}
