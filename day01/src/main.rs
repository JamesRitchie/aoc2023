use std::{path::PathBuf, process};

use clap::Parser;

use day01::run;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long)]
    part_two: bool,

    puzzle_input_path: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli.puzzle_input_path, cli.part_two) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
