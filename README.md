# Advent of Code 2023
![Build Status](https://github.com/JamesRitchie/aoc2023/actions/workflows/rust.yml/badge.svg)

My solutions to [Advent of Code 2023](https://adventofcode.com/2023), implemented in Rust.
This is the first project I've ever written in Rust that wasn't a tutorial, so whilst I'm trying to keep it idiomatic it might very well not be perfect!
This project is structured as a [Cargo workspace](https://doc.rust-lang.org/cargo/reference/workspaces.html), with each day's solution as a separate package.

## Usage

To run part one of a particular day's puzzle from the workspace root:
```shell
cargo run -p day01 -- puzzle_input_file.txt
```
(You'll need to provide your own puzzle input.)

To run part two add the `--part-two` flag:
```shell
cargo run -p day01 -- --part-two puzzle_input_file.txt
```

To run tests for every solution:
```shell
cargo test
```



