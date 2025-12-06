# Advent of Code Solutions 2025 🎄

Rust solutions for Advent of Code puzzles with a beautiful **Ratatui TUI** interface and automatic day discovery.

## ✨ Features

- 🎨 **Beautiful TUI** - Full-featured terminal UI with Ratatui
- 🔍 **Auto-Discovery** - Automatically detects available days from source files
- 📝 **Smart Parsing** - Extracts day titles from code comments
- 🚀 **CLI Mode** - Quick command-line interface for scripting
- 📁 **Flexible Input** - Support for files, default files, or stdin
- 🎯 **Zero Config** - No need to manually register days in main.rs
- ⚡ **Fast** - Written in Rust for maximum performance

## 🚀 Quick Start

```bash
# Clone and build
git clone <your-repo>
cd adventcode
cargo build --release

# Run the interactive TUI (just run with no args!)
cargo run

# Or run a specific day directly
cargo run -- --day 5 --part2
```

## 🎨 Screenshots

### Main Menu
```
╔════════════════════════════════════════╗
║                                        ║
║    🎄 Advent of Code Runner 🎄        ║
║                                        ║
╚════════════════════════════════════════╝

┌ Available Days ───────────────────────┐
│ ▶ Day  1: COMBINATION LOCK [✓]       │
│   Day  2: INVALID ID DETECTION [✓]   │
│   Day  3: LOBBY BATTERIES [✓]        │
│   Day  4: PRINTING DEPARTMENT [✓]    │
│   Day  5: CAFETERIA [✓]              │
└───────────────────────────────────────┘

↑↓: Navigate | Enter: Select | q: Quit
```

### Part Selection
```
┌──────────────────────────────────────┐
│      Day 5: CAFETERIA                │
└──────────────────────────────────────┘

┌ Select Part ─────────────────────────┐
│                                      │
│         ▶ Part 1                     │
│                                      │
│           Part 2                     │
└──────────────────────────────────────┘

↑↓: Select Part | Enter: Run | Backspace: Back | q: Quit
```

## 📖 Usage

### 🎨 Interactive TUI Mode (Recommended)

Simply run without any arguments:

```bash
cargo run
```

**Navigation:**
- `↑`/`↓` or `k`/`j` - Navigate through days/parts
- `Enter` - Select day or run puzzle
- `Backspace` - Go back to day selection
- `q` or `Esc` - Quit

The TUI will show:
- All available days with their titles (auto-detected!)
- Input file status (✓ if exists, ✗ if missing)
- Beautiful interface with syntax highlighting

### ⚡ Command-Line Mode

Run a specific day with explicit arguments:

```bash
# Part 1 with default input file (day1.txt)
cargo run -- --day 1

# Part 2 with custom input file
cargo run -- --day 1 --file input.txt --part2

# Quiet mode (no formatting, just output)
cargo run -- --day 1 --file input.txt --quiet
```

### 🎯 Short Flags

```bash
cargo run -- -d 1              # Day 1, Part 1, default file
cargo run -- -d 1 -2           # Day 1, Part 2, default file
cargo run -- -d 1 -f input.txt # Day 1, Part 1, custom file
cargo run -- -d 5 -2 -q        # Day 5, Part 2, quiet mode
```

### 📥 Using Stdin

Pipe input directly:

```bash
cat input.txt | cargo run -- -d 1 -q
echo "L68\nR30" | cargo run -- -d 1 -2 -q
cargo run -- -d 1 -q < input.txt
```

### ❓ Help

View all available options:

```bash
cargo run -- --help
```

## 🔧 Adding New Days - It's Automatic! 🎉

**No more manual registration!** Just create your day file and it's automatically detected.

### 1. Create the solution file

Create `src/dayN.rs` with the proper header format:

```rust
// ============================================================================
// DAY N: YOUR TITLE HERE
// ============================================================================
//
// PROBLEM OVERVIEW:
// -----------------
// Description of the problem...
//
// Part 1: What part 1 asks for
// Part 2: What part 2 asks for
//
// ============================================================================

pub fn solve(input: &str, part2: bool) {
    if part2 {
        // Part 2 logic
        println!("{}", result);
    } else {
        // Part 1 logic
        println!("{}", result);
    }
}
```

**Important:** The title is extracted from the comment `// DAY N: YOUR TITLE`

### 2. Add module declaration to main.rs

Just add ONE line to the top of `src/main.rs`:

```rust
mod dayN;
```

And add ONE match arm:

```rust
match day {
    // ...
    N => dayN::solve(&input, part2),
    _ => eprintln!("Day {} not implemented yet", day),
}
```

### 3. Add input file (optional)

Create `dayN.txt` with your puzzle input.

### 4. That's it! 🎉

The system will automatically:
- ✅ Detect the new day exists
- ✅ Extract the title from comments
- ✅ Check if input file exists
- ✅ Show it in the TUI menu
- ✅ Allow you to run it

**No need to update:**
- ❌ Day lists
- ❌ Menu items
- ❌ Title arrays
- ❌ Configuration files

## 🎯 How Auto-Discovery Works

The system scans `src/` for files matching `day*.rs` (day1.rs through day25.rs) and:

1. **Detects existence** - Checks which day files are present
2. **Extracts titles** - Parses the header comment `// DAY N: TITLE`
3. **Checks input** - Verifies if `dayN.txt` exists
4. **Populates TUI** - Automatically shows all found days

### Title Format

The title is extracted using regex pattern:
```regex
// DAY \d+: (.+)
```

Examples:
```rust
// DAY 5: CAFETERIA           → Title: "CAFETERIA"
// DAY 1: COMBINATION LOCK    → Title: "COMBINATION LOCK"
// DAY 10: SUPER COOL PUZZLE  → Title: "SUPER COOL PUZZLE"
```

## 📁 Input Files

By default, the program looks for `dayN.txt` files:

1. **Use default files**: Place input in `day1.txt`, `day2.txt`, etc.
2. **Specify custom file**: Use `--file custom.txt`
3. **Use stdin**: Pipe input when file doesn't exist

The TUI shows input status:
- `[✓]` - Input file exists
- `[✗]` - No input file (will prompt or use stdin)

## 🎨 Output Examples

### With TUI/CLI formatting:
```
────────────────────────────────────────────────────────────
🎄 Day 5: CAFETERIA │ Part 2
────────────────────────────────────────────────────────────
Result: 357907198933892
```

### Quiet mode:
```
357907198933892
```

## 📂 Project Structure

```
adventcode/
├── src/
│   ├── main.rs      # Auto-discovering CLI and TUI
│   ├── day1.rs      # Day 1: COMBINATION LOCK
│   ├── day2.rs      # Day 2: INVALID ID DETECTION
│   ├── day3.rs      # Day 3: LOBBY BATTERIES
│   ├── day4.rs      # Day 4: PRINTING DEPARTMENT
│   ├── day5.rs      # Day 5: CAFETERIA
│   └── ...          # Add more days as needed
├── day1.txt         # Input files (optional)
├── day2.txt
├── day3.txt
├── day4.txt
├── day5.txt
├── demo.sh          # Demo script
├── Cargo.toml
└── README.md
```

## 🎯 Solutions

| Day | Title | Part 1 | Part 2 | Status |
|-----|-------|--------|--------|---------|
| 1 | Combination Lock | ⭐ | ⭐ | ✅ |
| 2 | Invalid ID Detection | ⭐ | ⭐ | ✅ |
| 3 | Lobby Batteries | ⭐ | ⭐ | ✅ |
| 4 | Printing Department | ⭐ | ⭐ | ✅ |
| 5 | Cafeteria | ⭐ | ⭐ | ✅ |

### Brief Descriptions

- **Day 1: COMBINATION LOCK** - Circular dial rotation with position tracking
- **Day 2: INVALID ID DETECTION** - Finding repeated digit sequences in numeric ranges  
- **Day 3: LOBBY BATTERIES** - Maximum joltage from battery digit combinations
- **Day 4: PRINTING DEPARTMENT** - Grid-based paper roll accessibility counting
- **Day 5: CAFETERIA** - Fresh ingredient ID range merging and counting

## 📦 Dependencies

- [clap](https://github.com/clap-rs/clap) (v4.5) - Command-line argument parsing
- [ratatui](https://github.com/ratatui-org/ratatui) (v0.29) - Terminal user interface library
- [crossterm](https://github.com/crossterm-rs/crossterm) (v0.28) - Terminal manipulation
- [regex](https://github.com/rust-lang/regex) (v1.11) - Pattern matching for auto-discovery

## 🎓 What Makes This Special

### 🔍 Auto-Discovery System
Unlike traditional Advent of Code runners, this project:
- **No manual registration** - Just create `dayN.rs` and it's detected
- **Smart title extraction** - Parses your code comments automatically
- **Input validation** - Shows which days have input files ready

### 🎨 Beautiful TUI with Ratatui
- Professional terminal interface
- Smooth keyboard navigation
- Color-coded status indicators
- Responsive layout

### ⚡ Developer-Friendly
- Add days in seconds, not minutes
- Consistent code structure
- Comprehensive documentation in each file
- Easy to extend and maintain

## 🎓 Learning Resources

This project demonstrates:
- ✅ Rust module system and project structure
- ✅ File system operations and regex parsing
- ✅ TUI development with Ratatui
- ✅ CLI design with clap
- ✅ Pattern matching and error handling
- ✅ Clean code organization

## 📝 License

MIT

## 🤝 Contributing

Feel free to:
- Add more days (it's automatic!)
- Improve solutions
- Enhance the TUI
- Add features
- Fix bugs

---

Made with ❤️ and 🦀 Rust