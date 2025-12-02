// ============================================================================
// DAY 1: COMBINATION LOCK
// ============================================================================
//
// PROBLEM OVERVIEW:
// -----------------
// We have a circular dial with positions 0-99 that wraps around.
// Starting at position 50, we follow rotation instructions (L/R + distance).
// 
// Part 1: Count how many times we END at position 0 after each instruction.
// Part 2: Count how many times we PASS THROUGH position 0 during rotations.
//
// INPUT FORMAT:
// Each line contains a direction (L or R) followed by a distance number.
// Example: "L49" means rotate left 49 positions
//          "R24" means rotate right 24 positions
//
// ============================================================================

/// Main solver for Day 1
/// 
/// The dial is circular with 100 positions (0-99):
///   ... 98 - 99 - 0 - 1 - 2 ...
///        ↑________|________|
///        (wraps around)
pub fn solve(input: &str, part2: bool) {
    // Start at position 50 (given in problem)
    let mut pos: i32 = 50;
    
    // Count how many times we hit position 0
    let mut zero_hits: i32 = 0;

    // Process each rotation instruction
    for raw_line in input.lines() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        // Parse instruction: first char is direction, rest is distance
        // Example: "L49" → dir = "L", dist = 49
        let (dir, rest) = line.split_at(1);
        let dist: i32 = rest.parse().expect("invalid distance");

        if part2 {
            // ================================================================
            // PART 2: Count every click that passes through 0
            // ================================================================
            //
            // Key difference from Part 1:
            // - Part 1 only checks the FINAL position after each instruction
            // - Part 2 checks EACH INDIVIDUAL CLICK during the rotation
            //
            // Example: Starting at position 2, instruction "L5"
            //   Part 1: 2 → 97 (jumps directly, checks once)
            //   Part 2: 2 → 1 → 0 → 99 → 98 → 97 (checks 5 times)
            //                    ↑
            //                 Found it!
            
            // Simulate each individual click
            for _ in 0..dist {
                match dir {
                    "L" => {
                        // Rotate left (counter-clockwise, decrease position)
                        pos -= 1;
                        if pos < 0 {
                            // Wrap around: -1 becomes 99
                            pos = 99;
                        }
                    }
                    "R" => {
                        // Rotate right (clockwise, increase position)
                        pos += 1;
                        if pos > 99 {
                            // Wrap around: 100 becomes 0
                            pos = 0;
                        }
                    }
                    _ => panic!("unknown direction: {dir}"),
                }
                
                // Check if this individual click landed on 0
                if pos == 0 {
                    zero_hits += 1;
                }
            }
        } else {
            // ================================================================
            // PART 1: Only count final position after each instruction
            // ================================================================
            //
            // Strategy: Jump directly to the final position
            // - Don't simulate individual clicks
            // - Just calculate where we end up and check once
            //
            // Example: position 50, instruction "L60"
            //   50 - 60 = -10
            //   -10 % 100 = -10
            //   (-10 + 100) % 100 = 90  ← final position
            
            // Calculate new position based on direction
            match dir {
                "L" => pos -= dist,  // Left decreases position
                "R" => pos += dist,  // Right increases position
                _ => panic!("unknown direction: {dir}"),
            }

            // Handle wrapping with modulo arithmetic
            // The formula ((pos % 100) + 100) % 100 correctly handles negatives:
            //   pos = 105  →  (105 % 100 + 100) % 100 = 5
            //   pos = -10  →  (-10 % 100 + 100) % 100 = 90
            pos = ((pos % 100) + 100) % 100;

            // Check if we ended at position 0
            if pos == 0 {
                zero_hits += 1;
            }
        }
    }

    // The password is the total count of times we hit position 0
    println!("Password: {}", zero_hits);
}
