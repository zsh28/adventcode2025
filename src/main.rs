mod day1;

use clap::Parser;
use std::fs;
use std::io::{self, Read};

#[derive(Parser)]
#[command(name = "adventcode")]
#[command(about = "Advent of Code solutions", long_about = None)]
struct Cli {
    /// Day to run (e.g., 1, 2, 3...)
    #[arg(short, long)]
    day: u8,

    /// Input file path (if not provided, reads from stdin)
    #[arg(short, long)]
    file: Option<String>,

    /// Run part 2 of the puzzle
    #[arg(short = '2', long)]
    part2: bool,
}

fn main() {
    let cli = Cli::parse();

    // Read input from file or stdin
    let input = if let Some(file_path) = cli.file {
        fs::read_to_string(&file_path)
            .unwrap_or_else(|_| panic!("Failed to read file: {}", file_path))
    } else {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .expect("Failed to read from stdin");
        buffer
    };

    // Run the appropriate day
    match cli.day {
        1 => day1::solve(&input, cli.part2),
        _ => eprintln!("Day {} not implemented yet", cli.day),
    }
}