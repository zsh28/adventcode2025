# Advent of Code Solutions 2025

Rust solutions for Advent of Code puzzles with a flexible CLI interface.

## Features

- Command-line interface to run specific days
- Support for both Part 1 and Part 2 of puzzles
- Input from files or stdin
- Easy to extend for new days

## Installation

Ensure you have Rust installed, then build the project:

```bash
cargo build --release
```

## Usage

### Basic Usage

Run a specific day with an input file:

```bash
cargo run -- --day 1 --file input.txt
```

Run Part 2 of a puzzle:

```bash
cargo run -- --day 1 --file input.txt --part2
```

### Short Flags

```bash
cargo run -- -d 1 -f input.txt        # Part 1
cargo run -- -d 1 -f input.txt -2     # Part 2
```

### Using Stdin

You can also pipe input directly:

```bash
cat input.txt | cargo run -- --day 1
echo "L68\nR30" | cargo run -- -d 1 -2
cargo run -- -d 1 < input.txt
```

### Help

View all available options:

```bash
cargo run -- --help
```

## Project Structure

```
adventcode/
├── src/
│   ├── main.rs      # CLI and day routing
│   ├── day1.rs      # Day 1 solution
│   ├── day2.rs      # Day 2 solution (add as needed)
│   └── ...
├── Cargo.toml
└── README.md
```

## Adding New Days

To add a new day's solution:

1. Create a new file `src/dayN.rs` with a public `solve` function:

```rust
pub fn solve(input: &str, part2: bool) {
    // Your solution here
    if part2 {
        // Part 2 logic
    } else {
        // Part 1 logic
    }
    println!("Answer: {}", result);
}
```

2. Add the module to `src/main.rs`:

```rust
mod dayN;
```

3. Add a match arm in `main.rs`:

```rust
match cli.day {
    1 => day1::solve(&input, cli.part2),
    N => dayN::solve(&input, cli.part2),
    _ => eprintln!("Day {} not implemented yet", cli.day),
}
```

## Solutions

- **Day 1**: Secret Entrance - Safe dial puzzle with rotation counting

## Dependencies

- [clap](https://github.com/clap-rs/clap) - Command-line argument parsing