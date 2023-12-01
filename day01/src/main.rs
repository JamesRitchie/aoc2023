use std::path::PathBuf;

use clap::Parser;

use day01::run;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {

    #[arg(short, long)]
    part_two: bool,

    puzzle_input_path: PathBuf,
}


fn main() {
    let cli = Cli::parse();

    run(cli.puzzle_input_path, cli.part_two);
}
