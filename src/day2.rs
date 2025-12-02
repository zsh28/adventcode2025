// ============================================================================
// DAY 2: INVALID ID DETECTION
// ============================================================================
//
// PROBLEM OVERVIEW:
// -----------------
// We need to find "invalid IDs" within given numeric ranges.
//
// Part 1: An ID is invalid if it's a digit sequence repeated exactly twice.
//         Examples: 11 (1+1), 6464 (64+64), 123123 (123+123)
//
// Part 2: An ID is invalid if it's a digit sequence repeated at least twice.
//         Examples: 111 (1 three times), 12341234 (1234 twice),
//                   1212121212 (12 five times)
//
// ============================================================================

/// Represents an inclusive numeric range [start, end]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    start: u64,
    end: u64,
}

/// Parses comma-separated ranges in format "start-end,start-end,..."
/// 
/// Example input: "11-22,95-115,998-1012"
/// Returns: Vec<Range> with parsed start and end values
fn parse_ranges(input: &str) -> Vec<Range> {
    let cleaned = input.replace("\n", "").replace(" ", "");
    let mut ranges = Vec::new();
    
    for part in cleaned.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        
        // Split on '-' to get start and end values
        if let Some((a, b)) = part.split_once('-') {
            if let (Ok(start), Ok(end)) = (a.parse::<u64>(), b.parse::<u64>()) {
                ranges.push(Range { start, end });
            }
        }
    }
    
    ranges
}

/// Merges overlapping and adjacent ranges to optimize lookup performance.
/// 
/// Example: [11-22, 20-30, 95-115] becomes [11-30, 95-115]
/// 
/// This reduces the number of ranges we need to check and allows for
/// efficient binary search in the in_merged_ranges function.
fn merge_ranges(ranges: &[Range]) -> Vec<Range> {
    if ranges.is_empty() {
        return Vec::new();
    }
    
    // Sort ranges by start position
    let mut sorted = ranges.to_vec();
    sorted.sort();
    
    let mut merged = vec![sorted[0]];
    
    // Merge overlapping or adjacent ranges
    for range in sorted.iter().skip(1) {
        let last_idx = merged.len() - 1;
        // If current range overlaps or is adjacent to the last merged range
        if range.start <= merged[last_idx].end + 1 {
            // Extend the last merged range
            merged[last_idx].end = merged[last_idx].end.max(range.end);
        } else {
            // Start a new merged range
            merged.push(*range);
        }
    }
    
    merged
}

/// Checks if a number exists within any of the merged ranges using binary search.
/// 
/// Time complexity: O(log n) where n is the number of ranges
/// 
/// This is much faster than iterating through all ranges linearly,
/// especially when there are many ranges.
fn in_merged_ranges(x: u64, merged_ranges: &[Range]) -> bool {
    let mut lo = 0;
    let mut hi = merged_ranges.len();
    
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        let range = merged_ranges[mid];
        
        if x < range.start {
            // x is smaller, search left half
            hi = mid;
        } else if x > range.end {
            // x is larger, search right half
            lo = mid + 1;
        } else {
            // x is within this range
            return true;
        }
    }
    
    false
}

/// Checks if a number consists of a pattern repeated at least twice (Part 2).
/// 
/// Algorithm:
/// 1. Convert number to string
/// 2. Try all possible pattern lengths from 1 to len/2
/// 3. For each pattern length, check if the entire string is that pattern repeated
/// 
/// Examples:
/// - 111 = "1" repeated 3 times → true
/// - 1212 = "12" repeated 2 times → true
/// - 12341234 = "1234" repeated 2 times → true
/// - 1234 = no valid repetition → false
/// 
/// Time complexity: O(n²) where n is the number of digits
fn is_invalid_part2(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();
    
    // Try all possible pattern lengths from 1 to len/2
    // (pattern must be repeated at least twice, so max length is len/2)
    for pattern_len in 1..=(len / 2) {
        // Only consider pattern lengths that divide evenly into total length
        if len % pattern_len != 0 {
            continue;
        }
        
        let repetitions = len / pattern_len;
        // Must have at least 2 repetitions
        if repetitions < 2 {
            continue;
        }
        
        // Extract the pattern (first pattern_len characters)
        let pattern = &s[..pattern_len];
        let mut is_valid = true;
        
        // Check if every subsequent segment matches the pattern
        for i in 1..repetitions {
            let start = i * pattern_len;
            let end = start + pattern_len;
            if &s[start..end] != pattern {
                is_valid = false;
                break;
            }
        }
        
        // If we found a valid repetition, the number is invalid
        if is_valid {
            return true;
        }
    }
    
    false
}

/// PART 1 SOLUTION: Find sum of IDs with digit sequence repeated exactly twice
/// 
/// STRATEGY: Generate candidates instead of checking every number in ranges
/// 
/// Why this is efficient:
/// - Instead of checking billions of numbers, we generate only valid candidates
/// - For a max value of 10 billion (10 digits), we only check ~111,111 candidates
///   (all numbers from 1-9, 10-99, 100-999, 1000-9999, 10000-99999)
/// 
/// Algorithm:
/// 1. Find the maximum upper bound across all ranges
/// 2. Determine max digit length needed
/// 3. For each even total length (2, 4, 6, 8, ...):
///    - Generate all patterns of half that length (avoiding leading zeros)
///    - Form the doubled number by concatenating pattern with itself
///    - Check if it falls within any range using binary search
/// 
/// Example for length 4:
/// - Half length = 2
/// - Generate: 10, 11, 12, ..., 99 (all 2-digit numbers)
/// - Create: 1010, 1111, 1212, ..., 9999
/// - Check which ones fall in ranges
/// 
/// Time complexity: O(k * log n) where k is number of candidates, n is number of ranges
fn sum_invalid_ids(ranges_str: &str) -> u64 {
    let ranges = parse_ranges(ranges_str);
    if ranges.is_empty() {
        return 0;
    }
    
    // Merge ranges for efficient lookup
    let merged = merge_ranges(&ranges);
    let max_upper = merged.iter().map(|r| r.end).max().unwrap_or(0);
    let max_digits = max_upper.to_string().len();
    
    let mut invalid_sum = 0u64;
    
    // Only check even lengths (since we're doubling patterns)
    for total_len in (2..=max_digits).step_by(2) {
        let half = total_len / 2;
        
        // Generate all patterns with 'half' digits (no leading zeros)
        // Example: for half=2, generate 10 to 99
        let start = 10u64.pow((half - 1) as u32);  // First number with 'half' digits
        let end = 10u64.pow(half as u32);           // First number with 'half+1' digits
        
        for t in start..end {
            let s = t.to_string();
            // Create the doubled pattern: "64" + "64" = "6464"
            let doubled = format!("{}{}", s, s);
            
            if let Ok(num) = doubled.parse::<u64>() {
                // Early exit: if we've exceeded max_upper, no point continuing
                if num > max_upper {
                    break;
                }
                
                // Check if this invalid ID is in any of our ranges
                if in_merged_ranges(num, &merged) {
                    invalid_sum += num;
                }
            }
        }
    }
    
    invalid_sum
}

/// PART 2 SOLUTION: Find sum of IDs with digit sequence repeated at least twice
/// 
/// STRATEGY: Check every number in the ranges
/// 
/// Why we can't use the Part 1 generation approach:
/// - Part 1 only needs patterns repeated exactly twice (even lengths only)
/// - Part 2 needs patterns repeated 2+ times (any length divisible by pattern)
/// - Examples: 111 (1×3), 1212 (12×2), 123123123 (123×3)
/// - It's harder to efficiently generate all possible combinations
/// 
/// Algorithm:
/// 1. Parse and merge the ranges
/// 2. Iterate through every number in every range
/// 3. For each number, check if it's a repeated pattern (using is_invalid_part2)
/// 4. Sum up all invalid IDs found
/// 
/// Trade-off:
/// - This is slower than Part 1 (brute force vs generation)
/// - But it's simpler and handles all repetition counts
/// - For typical AoC inputs, performance is still acceptable
/// 
/// Time complexity: O(R * D²) where R is total range size, D is digits per number
fn sum_invalid_ids_part2(ranges_str: &str) -> u64 {
    let ranges = parse_ranges(ranges_str);
    if ranges.is_empty() {
        return 0;
    }
    
    // Merge ranges to avoid checking duplicates
    let merged = merge_ranges(&ranges);
    let mut invalid_sum = 0u64;
    
    // Check every number in every range
    for range in &merged {
        for num in range.start..=range.end {
            if is_invalid_part2(num) {
                invalid_sum += num;
            }
        }
    }
    
    invalid_sum
}

/// Main entry point for Day 2 solution
pub fn solve(input: &str, part2: bool) {
    let result = if part2 {
        sum_invalid_ids_part2(input)
    } else {
        sum_invalid_ids(input)
    };
    println!("Sum of invalid IDs: {}", result);
}
