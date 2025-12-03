// ============================================================================
// DAY 3: LOBBY BATTERIES
// ============================================================================
//
// PROBLEM OVERVIEW:
// -----------------
// We have batteries arranged in banks, where each battery has a joltage (1-9).
// Each line represents one bank of batteries.
// 
// Part 1: Find the maximum joltage each bank can produce by turning on exactly
//         two batteries. The joltage is the number formed by the two digits
//         (in order, no rearrangement).
//
// Part 2: Find the maximum joltage by turning on exactly twelve batteries.
//         The joltage is the 12-digit number formed by the selected digits.
//
// STRATEGY FOR PART 2:
// Use a greedy approach: at each position in the result (left to right),
// choose the largest digit from the remaining input, ensuring we have enough
// digits left to complete the 12-digit number.
//
// For position i (0-indexed) in the result:
// - We need (12 - i) more digits total
// - We can search up to position: input.len() - (12 - i) + 1
// - Pick the maximum digit in that range
//
// Example: "987654321111111" (15 digits), need 12
// - Position 0: search indices 0..4 (need 12 more, so 15-12+1=4), pick 9 at index 0
// - Position 1: search indices 1..5, pick 8 at index 1
// - Continue this way...
//
// ============================================================================

/// Find the largest k-digit number by selecting k digits from the input
/// while maintaining their relative order.
fn find_max_k_digits(digits: &[u32], k: usize) -> u64 {
    if k == 0 || digits.is_empty() {
        return 0;
    }
    
    if k > digits.len() {
        // Can't select k digits from fewer than k available
        return 0;
    }
    
    let mut result = 0u64;
    let mut start_idx = 0;
    
    for i in 0..k {
        // How many more digits do we need after this one?
        let remaining_needed = k - i - 1;
        
        // Latest index we can pick from and still have enough digits left
        let search_end = digits.len() - remaining_needed;
        
        // Find the maximum digit in the valid range
        let mut max_digit = digits[start_idx];
        let mut max_idx = start_idx;
        
        for j in start_idx..search_end {
            if digits[j] > max_digit {
                max_digit = digits[j];
                max_idx = j;
            }
        }
        
        // Add this digit to our result
        result = result * 10 + max_digit as u64;
        
        // Next search starts after the digit we just picked
        start_idx = max_idx + 1;
    }
    
    result
}

/// Main solver for Day 3
pub fn solve(input: &str, part2: bool) {
    let mut total_joltage = 0u64;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Convert line to vector of digit values
        let digits: Vec<u32> = line
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect();

        if part2 {
            // Part 2: Select 12 batteries
            if digits.len() < 12 {
                continue; // Need at least 12 batteries
            }
            
            let max_joltage = find_max_k_digits(&digits, 12);
            total_joltage += max_joltage;
        } else {
            // Part 1: Select 2 batteries
            if digits.len() < 2 {
                continue; // Need at least 2 batteries
            }

            // Find maximum joltage by checking all pairs
            let mut max_joltage = 0u32;

            for i in 0..digits.len() {
                for j in (i + 1)..digits.len() {
                    let joltage = digits[i] * 10 + digits[j];
                    max_joltage = max_joltage.max(joltage);
                }
            }

            total_joltage += max_joltage as u64;
        }
    }

    println!("Total output joltage: {}", total_joltage);
}
