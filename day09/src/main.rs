use clap::Parser;
use std::process;

use day09::run;
use util::Cli;

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli.puzzle_input_path, cli.part_two) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
