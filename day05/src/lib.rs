use std::{error::Error, fs, path::PathBuf, str::FromStr};

/// Represents seeds covering a range.
/// Range is half-inclusive.
struct SeedRange {
    start: i64,
    end: i64,
}
/// Represents a mapping from a particular source range to a destination range.
/// Ranges are half-inclusive.
struct MapRange {
    src_start: i64,
    src_end: i64,
    dst_start: i64,
    dst_end: i64,
}

impl FromStr for MapRange {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .split(" ")
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let d_start = values[0];
        let s_start = values[1];
        let length = values[2];

        Ok(Self {
            src_start: s_start,
            src_end: s_start + length,
            dst_start: d_start,
            dst_end: d_start + length,
        })
    }
}

/// Collection of MapRanges.
struct SeedMap {
    map_ranges: Vec<MapRange>,
}

impl FromStr for SeedMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ranges = s
            .split(":\n")
            .last()
            .unwrap()
            .lines()
            .map(|s| MapRange::from_str(s).unwrap())
            .collect::<Vec<_>>();
        ranges.sort_by_key(|r| r.src_start);

        Ok(Self { map_ranges: ranges })
    }
}

impl SeedMap {
    /// Apply transformation to a single seed number.
    fn map_seed(&self, seed: i64) -> i64 {
        let map_range = self
            .map_ranges
            .iter()
            .filter(|r| seed >= r.src_start && seed < r.src_end)
            .next();

        match map_range {
            Some(m) => seed - m.src_start + m.dst_start,
            None => seed,
        }
    }

    /// Apply transformation to a range of seed numbers.
    /// As the map ranges may overlap the seed ranges, multiple seed ranges can be returned.
    fn map_seed_range(&self, src_range: SeedRange) -> Vec<SeedRange> {
        let mut dst_ranges: Vec<SeedRange> = vec![];

        for m in &self.map_ranges {
            // map_ranges is sorted
            if src_range.start < m.src_start && src_range.end < m.src_start {
                // The src_range is entirely to the left of the map_range, so no transform needed.
                dst_ranges.push(src_range);
                return dst_ranges;
            } else if src_range.start < m.src_start && src_range.end >= m.src_start {
                // The src_range overlaps the map_range to the left, so split the src_range at the
                // left intersection.
                dst_ranges.push(SeedRange {
                    start: src_range.start,
                    end: m.src_start - 1,
                });
                dst_ranges.append(&mut self.map_seed_range(SeedRange {
                    start: m.src_start,
                    end: src_range.end,
                }));
                return dst_ranges;
            } else if src_range.start >= m.src_start && src_range.end < m.src_end {
                // The src_range is entirely contained by the map_range, so transform it.
                dst_ranges.push(SeedRange {
                    start: src_range.start - m.src_start + m.dst_start,
                    end: src_range.end - m.src_start + m.dst_start,
                });
                return dst_ranges;
            } else if src_range.start < m.src_end && src_range.end >= m.src_end {
                // The src range overlaps the map_range to the right, so split it at the right
                // intersection and transform the left split.
                dst_ranges.push(SeedRange {
                    start: src_range.start - m.src_start + m.dst_start,
                    end: m.dst_end,
                });
                dst_ranges.append(&mut self.map_seed_range(SeedRange {
                    start: m.src_end,
                    end: src_range.end,
                }));
                return dst_ranges;
            }
        }

        // If we got here the src_range is to the right of all the map ranges.
        dst_ranges.push(src_range);
        dst_ranges
    }

    /// Apply transformation to a set of seed ranges and concatenate the results together.
    fn map_seed_ranges(&self, seed_ranges: Vec<SeedRange>) -> Vec<SeedRange> {
        seed_ranges
            .into_iter()
            .flat_map(|s| self.map_seed_range(s))
            .collect::<Vec<_>>()
    }
}

pub fn compute_answer(puzzle_input: &str, part_two: bool) -> i64 {
    let (seed_list, map_list) = puzzle_input.split_once("\n\n").unwrap();

    let maps = map_list
        .split("\n\n")
        .map(|s| SeedMap::from_str(s).unwrap())
        .collect::<Vec<_>>();

    let seed_vec = seed_list[7..]
        .split(" ")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    if part_two {
        // Pair up the seed inputs to get the ranges.
        let seed_ranges = seed_vec
            .chunks(2)
            .map(|c| SeedRange {
                start: c[0],
                end: c[0] + c[1],
            })
            .collect::<Vec<_>>();

        maps.iter()
            .fold(seed_ranges, |s, m| m.map_seed_ranges(s))
            .iter()
            .map(|s| s.start)
            .min()
            .unwrap()
    } else {
        seed_vec
            .into_iter()
            .map(|seed| maps.iter().fold(seed, |s, m| m.map_seed(s)))
            .min()
            .unwrap()
    }
}

pub fn run(input_path: PathBuf, part_two: bool) -> Result<i64, Box<dyn Error>> {
    let puzzle_input = fs::read_to_string(input_path)?;
    let answer = compute_answer(&puzzle_input, part_two);
    println!("The answer is {answer}");
    Ok(answer)
}
