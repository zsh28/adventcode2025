// ============================================================================
// DAY 4: PRINTING DEPARTMENT
// ============================================================================
//
// PROBLEM OVERVIEW:
// -----------------
// We have a grid of paper rolls marked with '@' symbols.
// We need to find which rolls the forklifts can access.
//
// Part 1: A roll can be accessed if there are FEWER than 4 rolls of paper
//         in the eight adjacent positions (including diagonals).
//
// GRID LAYOUT:
// The 8 adjacent positions for any cell (r, c) are:
//
//     (r-1,c-1)  (r-1,c)  (r-1,c+1)    NW  N  NE
//     (r,c-1)    (r,c)    (r,c+1)       W  X   E
//     (r+1,c-1)  (r+1,c)  (r+1,c+1)    SW  S  SE
//
// EXAMPLE:
// Input grid:
//   ..@@.@@@@.
//   @@@.@.@.@@
//   @@@@@.@.@@
//
// Output (x marks accessible rolls):
//   ..xx.xx@x.
//   x@@.@.@.@@
//   @@@@@.x.@@
//
// ============================================================================

/// Parses the input grid into a 2D vector of characters
fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

/// Counts the number of '@' symbols in the 8 adjacent positions
/// 
/// Arguments:
/// - grid: The 2D grid of characters
/// - row: The row index of the cell to check
/// - col: The column index of the cell to check
/// 
/// Returns: The count of adjacent paper rolls (0-8)
fn count_adjacent_rolls(grid: &[Vec<char>], row: usize, col: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    // Define the 8 directions: N, NE, E, SE, S, SW, W, NW
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),  // Top row
        (0, -1),           (0, 1),   // Middle row (left and right)
        (1, -1),  (1, 0),  (1, 1),   // Bottom row
    ];

    for (dr, dc) in directions.iter() {
        // Calculate the new position
        let new_row = row as i32 + dr;
        let new_col = col as i32 + dc;

        // Check bounds
        if new_row >= 0 && new_row < rows as i32 && new_col >= 0 && new_col < cols as i32 {
            let nr = new_row as usize;
            let nc = new_col as usize;

            // Check if there's a paper roll at this position
            if grid[nr][nc] == '@' {
                count += 1;
            }
        }
    }

    count
}

/// PART 1 SOLUTION: Count accessible paper rolls
/// 
/// STRATEGY: Check each '@' cell and count adjacent rolls
/// 
/// Algorithm:
/// 1. Parse the input into a 2D grid
/// 2. For each cell containing '@':
///    a. Count how many '@' symbols are in the 8 adjacent cells
///    b. If the count is less than 4, this roll is accessible
/// 3. Return the total count of accessible rolls
/// 
/// Time complexity: O(R × C) where R is rows and C is columns
fn count_accessible_rolls(input: &str) -> usize {
    let grid = parse_grid(input);
    
    if grid.is_empty() {
        return 0;
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let mut accessible_count = 0;

    // Check each cell in the grid
    for row in 0..rows {
        for col in 0..cols {
            // Only check cells that contain a paper roll
            if grid[row][col] == '@' {
                let adjacent = count_adjacent_rolls(&grid, row, col);
                
                // Roll is accessible if fewer than 4 adjacent rolls
                if adjacent < 4 {
                    accessible_count += 1;
                }
            }
        }
    }

    accessible_count
}

/// PART 2 SOLUTION: Count total removable paper rolls through iterative removal
/// 
/// STRATEGY: Simulate the process of removing accessible rolls repeatedly
/// 
/// Algorithm:
/// 1. Parse the input into a mutable 2D grid
/// 2. Repeat until no more rolls can be removed:
///    a. Find all currently accessible rolls (adjacent count < 4)
///    b. If no accessible rolls found, stop
///    c. Remove all accessible rolls (replace '@' with '.')
///    d. Add the count to the running total
/// 3. Return the total count of removed rolls
/// 
/// Why we remove in batches:
/// - The problem shows removing all accessible rolls at once per iteration
/// - This matches the visualization where all 'x' marks appear simultaneously
/// - Removing one at a time could give different results (order matters)
/// 
/// Time complexity: O(I × R × C) where I is iterations, R is rows, C is columns
/// In practice, I is bounded by the total number of rolls
fn count_removable_rolls(input: &str) -> usize {
    let mut grid = parse_grid(input);
    
    if grid.is_empty() {
        return 0;
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let mut total_removed = 0;

    // Keep removing accessible rolls until none remain
    loop {
        // Find all accessible rolls in current state
        let mut accessible = Vec::new();
        
        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] == '@' {
                    let adjacent = count_adjacent_rolls(&grid, row, col);
                    if adjacent < 4 {
                        accessible.push((row, col));
                    }
                }
            }
        }

        // If no accessible rolls found, we're done
        if accessible.is_empty() {
            break;
        }

        // Remove all accessible rolls (replace with '.')
        for (row, col) in &accessible {
            grid[*row][*col] = '.';
        }

        // Add to total count
        total_removed += accessible.len();
    }

    total_removed
}

/// Main entry point for Day 4 solution
pub fn solve(input: &str, part2: bool) {
    if part2 {
        let result = count_removable_rolls(input);
        println!("Total removable rolls: {}", result);
    } else {
        let result = count_accessible_rolls(input);
        println!("Accessible rolls: {}", result);
    }
}
