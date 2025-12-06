// ============================================================================
// DAY 5: CAFETERIA
// ============================================================================
//
// PROBLEM OVERVIEW:
// -----------------
// The cafeteria has an inventory management system with fresh ingredient ranges
// and a list of available ingredients. We need to determine which ingredients
// are fresh based on ID ranges.
//
// Part 1: Count how many available ingredient IDs are fresh.
//         An ingredient ID is fresh if it falls within ANY of the fresh ranges.
//
// Part 2: Count the total number of ingredient IDs considered fresh by the
//         ranges themselves (ignoring the available ingredients list).
//         Overlapping ranges must be merged to avoid double-counting.
//
// INPUT FORMAT:
// -------------
// The input has two sections separated by a blank line:
//
// 1. Fresh ingredient ID ranges (inclusive):
//    Format: "start-end" where both start and end are included
//    Example: "3-5" means IDs 3, 4, and 5 are all fresh
//    Ranges can overlap: both "10-14" and "12-18" can exist
//
// 2. Available ingredient IDs (Part 1 only):
//    One ID per line
//
// EXAMPLE:
// --------
// Input:
//   3-5
//   10-14
//   16-20
//   12-18
//   
//   1
//   5
//   8
//   11
//   17
//   32
//
// Part 1 Analysis:
//   ID 1:  spoiled (not in any range)
//   ID 5:  fresh (in range 3-5)
//   ID 8:  spoiled (not in any range)
//   ID 11: fresh (in range 10-14)
//   ID 17: fresh (in ranges 12-18 and 16-20)
//   ID 32: spoiled (not in any range)
//   Result: 3 fresh ingredients
//
// Part 2 Analysis:
//   Ranges: [3-5], [10-14], [12-18], [16-20]
//   After merging overlaps: [3-5], [10-20]
//   Range 3-5 contains: 3, 4, 5 (3 IDs)
//   Range 10-20 contains: 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20 (11 IDs)
//   Result: 14 total fresh ingredient IDs
//
// KEY ALGORITHM (Part 2):
// ------------------------
// 1. Sort ranges by start position
// 2. Iterate through sorted ranges:
//    - If current range overlaps or is adjacent to the previous range, merge them
//    - Otherwise, save the previous range and start a new one
// 3. Count total IDs in all merged ranges: sum of (end - start + 1)
//
// MERGING LOGIC:
// Two ranges (a, b) and (c, d) can be merged if c <= b + 1
// (they overlap or are adjacent)
// Example: [10, 14] and [12, 18] merge to [10, 18]
//          [10, 14] and [15, 18] merge to [10, 18] (adjacent)
//          [10, 14] and [16, 18] don't merge (gap at 15)
//
// ============================================================================

pub fn solve(input: &str, part2: bool) {
    if part2 {
        solve_part2(input);
    } else {
        solve_part1(input);
    }
}

/// Part 1: Count how many available ingredient IDs are fresh
/// An ingredient ID is fresh if it falls within any of the fresh ranges
fn solve_part1(input: &str) {
    let lines: Vec<&str> = input.lines().collect();
    
    // Find the blank line that separates ranges from ingredient IDs
    let blank_line_idx = lines.iter().position(|&line| line.trim().is_empty())
        .expect("No blank line found in input");
    
    // Parse the fresh ingredient ranges (e.g., "3-5" means IDs 3, 4, 5 are fresh)
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    for line in &lines[..blank_line_idx] {
        if let Some((start, end)) = parse_range(line) {
            ranges.push((start, end));
        }
    }
    
    // Parse and check available ingredient IDs
    let mut fresh_count = 0;
    for line in &lines[blank_line_idx + 1..] {
        if line.trim().is_empty() {
            continue;
        }
        if let Ok(id) = line.trim().parse::<u64>() {
            // Check if this ID falls within any fresh range
            if is_fresh(id, &ranges) {
                fresh_count += 1;
            }
        }
    }
    
    println!("{}", fresh_count);
}

/// Part 2: Count total number of ingredient IDs considered fresh by the ranges
/// This means counting all IDs within the ranges (after merging overlapping ranges)
fn solve_part2(input: &str) {
    let lines: Vec<&str> = input.lines().collect();
    
    // Find the blank line that separates ranges from ingredient IDs
    let blank_line_idx = lines.iter().position(|&line| line.trim().is_empty())
        .expect("No blank line found in input");
    
    // Parse the fresh ingredient ranges
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    for line in &lines[..blank_line_idx] {
        if let Some((start, end)) = parse_range(line) {
            ranges.push((start, end));
        }
    }
    
    // Merge overlapping ranges to avoid double-counting
    let merged_ranges = merge_ranges(&mut ranges);
    
    // Count total IDs in all merged ranges
    let total_fresh: u64 = merged_ranges.iter()
        .map(|&(start, end)| end - start + 1)
        .sum();
    
    println!("{}", total_fresh);
}

/// Parse a range string like "3-5" into (3, 5)
fn parse_range(line: &str) -> Option<(u64, u64)> {
    let parts: Vec<&str> = line.split('-').collect();
    if parts.len() == 2 {
        let start = parts[0].parse::<u64>().ok()?;
        let end = parts[1].parse::<u64>().ok()?;
        Some((start, end))
    } else {
        None
    }
}

/// Check if an ingredient ID is fresh (falls within any range)
fn is_fresh(id: u64, ranges: &[(u64, u64)]) -> bool {
    ranges.iter().any(|&(start, end)| id >= start && id <= end)
}

/// Merge overlapping ranges to avoid counting IDs multiple times
/// For example: [(3,5), (10,14), (12,18)] becomes [(3,5), (10,18)]
fn merge_ranges(ranges: &mut [(u64, u64)]) -> Vec<(u64, u64)> {
    if ranges.is_empty() {
        return Vec::new();
    }
    
    // Sort ranges by start position
    ranges.sort_by_key(|&(start, _)| start);
    
    let mut merged: Vec<(u64, u64)> = Vec::new();
    let mut current = ranges[0];
    
    for &(start, end) in &ranges[1..] {
        // If ranges overlap or are adjacent, merge them
        if start <= current.1 + 1 {
            current.1 = current.1.max(end);
        } else {
            // No overlap, save current and start a new range
            merged.push(current);
            current = (start, end);
        }
    }
    
    // Don't forget the last range
    merged.push(current);
    merged
}