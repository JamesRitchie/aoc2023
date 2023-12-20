use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(long)]
    pub part_two: bool,

    pub puzzle_input_path: PathBuf,
}

pub fn transpose<T>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    (0..(matrix[0].len()))
        .map(|c| matrix.iter().map(|r| r[c].clone()).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}
