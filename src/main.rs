mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};
use regex::Regex;
use std::fs;
use std::io::{self, Read};

#[derive(Parser)]
#[command(name = "adventcode")]
#[command(about = "Advent of Code solutions", long_about = None)]
struct Cli {
    /// Day to run (e.g., 1, 2, 3...) - if not provided, interactive TUI is shown
    #[arg(short, long)]
    day: Option<u8>,

    /// Input file path (if not provided, uses default dayX.txt)
    #[arg(short, long)]
    file: Option<String>,

    /// Run part 2 of the puzzle
    #[arg(short = '2', long)]
    part2: bool,

    /// Run in non-interactive mode (no TUI, plain output)
    #[arg(short, long)]
    quiet: bool,
}

#[derive(Debug, Clone)]
struct DayInfo {
    number: u8,
    title: String,
    has_input: bool,
}

struct App {
    days: Vec<DayInfo>,
    selected_day: ListState,
    selected_part: usize, // 0 for Part 1, 1 for Part 2
    in_part_selection: bool,
}

impl App {
    fn new(days: Vec<DayInfo>) -> Self {
        let mut selected_day = ListState::default();
        if !days.is_empty() {
            selected_day.select(Some(0));
        }
        Self {
            days,
            selected_day,
            selected_part: 0,
            in_part_selection: false,
        }
    }

    fn next_day(&mut self) {
        let i = match self.selected_day.selected() {
            Some(i) => {
                if i >= self.days.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.selected_day.select(Some(i));
    }

    fn previous_day(&mut self) {
        let i = match self.selected_day.selected() {
            Some(i) => {
                if i == 0 {
                    self.days.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.selected_day.select(Some(i));
    }

    fn toggle_part(&mut self) {
        self.selected_part = 1 - self.selected_part;
    }

    fn get_selected_day(&self) -> Option<&DayInfo> {
        self.selected_day.selected().and_then(|i| self.days.get(i))
    }
}

fn main() {
    let cli = Cli::parse();

    // Discover available days
    let days = discover_days();

    if days.is_empty() {
        eprintln!("No day modules found!");
        std::process::exit(1);
    }

    // If day is specified, run directly
    if let Some(day) = cli.day {
        run_day(day, cli.part2, cli.file, cli.quiet);
    } else if cli.quiet {
        eprintln!("Error: --day is required when using --quiet mode");
        std::process::exit(1);
    } else {
        // Run TUI
        match run_tui(days) {
            Ok((day, part2)) => {
                // Clear screen and run the selected day
                println!("\n");
                run_day(day, part2, None, false);
            }
            Err(e) => {
                eprintln!("TUI error: {}", e);
                std::process::exit(1);
            }
        }
    }
}

/// Discover available days by reading the source directory
fn discover_days() -> Vec<DayInfo> {
    let mut days = Vec::new();

    // Check for day1.rs through day25.rs
    for day_num in 1..=25 {
        let source_file = format!("src/day{}.rs", day_num);
        if std::path::Path::new(&source_file).exists() {
            // Extract title from the file
            let title = extract_title_from_file(&source_file, day_num);
            let has_input = std::path::Path::new(&format!("day{}.txt", day_num)).exists();

            days.push(DayInfo {
                number: day_num,
                title,
                has_input,
            });
        }
    }

    days
}

/// Extract the day title from the source file header comment
fn extract_title_from_file(path: &str, day_num: u8) -> String {
    if let Ok(content) = fs::read_to_string(path) {
        // Look for pattern: // DAY N: TITLE
        let re = Regex::new(r"(?m)^//\s*DAY\s+\d+:\s*(.+?)\s*$").unwrap();
        if let Some(caps) = re.captures(&content) {
            return caps.get(1).unwrap().as_str().to_string();
        }
    }
    format!("Day {}", day_num)
}

/// Run the TUI and return the selected day and part
fn run_tui(days: Vec<DayInfo>) -> Result<(u8, bool), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new(days);
    let result = run_app(&mut terminal, &mut app)?;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result.ok_or_else(|| "No selection made".into())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> Result<Option<(u8, bool)>, Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            if app.in_part_selection {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(None),
                    KeyCode::Up | KeyCode::Char('k') => app.toggle_part(),
                    KeyCode::Down | KeyCode::Char('j') => app.toggle_part(),
                    KeyCode::Enter => {
                        if let Some(day) = app.get_selected_day() {
                            return Ok(Some((day.number, app.selected_part == 1)));
                        }
                    }
                    KeyCode::Backspace => app.in_part_selection = false,
                    _ => {}
                }
            } else {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(None),
                    KeyCode::Down | KeyCode::Char('j') => app.next_day(),
                    KeyCode::Up | KeyCode::Char('k') => app.previous_day(),
                    KeyCode::Enter => app.in_part_selection = true,
                    _ => {}
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7),  // Header
            Constraint::Min(10),     // Main content
            Constraint::Length(3),   // Footer
        ])
        .split(f.area());

    // Header
    let header = Paragraph::new(vec![
        Line::from(""),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("🎄 ", Style::default().fg(Color::Green)),
            Span::styled(
                "Advent of Code Runner",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" 🎄", Style::default().fg(Color::Green)),
        ])
        .alignment(Alignment::Center),
        Line::from(""),
        Line::from("  Select a day to run")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Gray)),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan)),
    );
    f.render_widget(header, chunks[0]);

    // Main content area
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(chunks[1]);

    if !app.in_part_selection {
        // Day selection
        render_day_list(f, app, main_chunks[0]);
        render_day_info(f, app, main_chunks[1]);
    } else {
        // Part selection
        render_part_selection(f, app, chunks[1]);
    }

    // Footer
    let footer_text = if app.in_part_selection {
        "↑↓: Select Part | Enter: Run | Backspace: Back | q: Quit"
    } else {
        "↑↓: Navigate | Enter: Select | q: Quit"
    };

    let footer = Paragraph::new(footer_text)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        );
    f.render_widget(footer, chunks[2]);
}

fn render_day_list(f: &mut Frame, app: &mut App, area: Rect) {
    let items: Vec<ListItem> = app
        .days
        .iter()
        .map(|day| {
            let status = if day.has_input { "✓" } else { "✗" };
            let content = format!("Day {:2}: {} [{}]", day.number, day.title, status);
            ListItem::new(content)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title("Available Days")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green)),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    f.render_stateful_widget(list, area, &mut app.selected_day);
}

fn render_day_info(f: &mut Frame, app: &App, area: Rect) {
    let info_text = if let Some(day) = app.get_selected_day() {
        let input_status = if day.has_input {
            format!("✓ Input file: day{}.txt", day.number)
        } else {
            format!("✗ No input file (day{}.txt missing)", day.number)
        };

        vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("Day: ", Style::default().fg(Color::Cyan)),
                Span::styled(
                    day.number.to_string(),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Title: ", Style::default().fg(Color::Cyan)),
                Span::raw(&day.title),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Status: ", Style::default().fg(Color::Cyan)),
                Span::raw(input_status),
            ]),
            Line::from(""),
            Line::from(""),
            Line::from(vec![
                Span::styled("Press ", Style::default().fg(Color::Gray)),
                Span::styled("Enter", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::styled(" to continue", Style::default().fg(Color::Gray)),
            ]),
        ]
    } else {
        vec![Line::from("No day selected")]
    };

    let info = Paragraph::new(info_text)
        .block(
            Block::default()
                .title("Details")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green)),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(info, area);
}

fn render_part_selection(f: &mut Frame, app: &App, area: Rect) {
    let day = app.get_selected_day().unwrap();

    // Center the selection box
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .split(area);

    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(15),
            Constraint::Percentage(70),
            Constraint::Percentage(15),
        ])
        .split(vertical_chunks[1]);

    let center_area = horizontal_chunks[1];

    let inner_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Length(9),  // Part selection
        ])
        .split(center_area);

    // Title
    let title = Paragraph::new(format!("Day {}: {}", day.number, day.title))
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green)),
        );
    f.render_widget(title, inner_chunks[0]);

    // Part selection with clear visual separation
    let part1_style = if app.selected_part == 0 {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
            .bg(Color::DarkGray)
    } else {
        Style::default().fg(Color::White)
    };

    let part2_style = if app.selected_part == 1 {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
            .bg(Color::DarkGray)
    } else {
        Style::default().fg(Color::White)
    };

    let part1_symbol = if app.selected_part == 0 { "▶ " } else { "  " };
    let part2_symbol = if app.selected_part == 1 { "▶ " } else { "  " };

    let text = vec![
        Line::from(""),
        Line::from("  Select which part to run:")
            .style(Style::default().fg(Color::Gray)),
        Line::from(""),
        Line::from(vec![
            Span::raw("      "),
            Span::styled(part1_symbol, part1_style),
            Span::styled("Part 1", part1_style),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("      "),
            Span::styled(part2_symbol, part2_style),
            Span::styled("Part 2", part2_style),
        ]),
    ];

    let parts_widget = Paragraph::new(text)
        .block(
            Block::default()
                .title(" Select Part ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Magenta)),
        );
    f.render_widget(parts_widget, inner_chunks[1]);
}

fn run_day(day: u8, part2: bool, file: Option<String>, quiet: bool) {
    // Determine input file path
    let input_file = file.unwrap_or_else(|| format!("day{}.txt", day));

    // Read input from file or stdin
    let input = if std::path::Path::new(&input_file).exists() {
        fs::read_to_string(&input_file)
            .unwrap_or_else(|_| panic!("Failed to read file: {}", input_file))
    } else {
        if !quiet {
            eprintln!("⚠ File '{}' not found, reading from stdin...", input_file);
        }
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .expect("Failed to read from stdin");
        buffer
    };

    // Print header in non-quiet mode
    if !quiet {
        let day_info = discover_days()
            .into_iter()
            .find(|d| d.number == day)
            .unwrap_or_else(|| DayInfo {
                number: day,
                title: format!("Day {}", day),
                has_input: false,
            });

        let part_name = if part2 { "Part 2" } else { "Part 1" };

        println!("{}", "─".repeat(60));
        println!(
            "🎄 Day {}: {} │ {}",
            day, day_info.title, part_name
        );
        println!("{}", "─".repeat(60));
        print!("Result: ");
    }

    match day {
        1 => day1::solve(&input, part2),
        2 => day2::solve(&input, part2),
        3 => day3::solve(&input, part2),
        4 => day4::solve(&input, part2),
        5 => day5::solve(&input, part2),
        _ => eprintln!("Day {} not implemented yet", day),
    }
}
