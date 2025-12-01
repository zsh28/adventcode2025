pub fn solve(input: &str, part2: bool) {
    let mut pos: i32 = 50; // starting position
    let mut zero_hits: i32 = 0;

    for raw_line in input.lines() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        let (dir, rest) = line.split_at(1);
        let dist: i32 = rest.parse().expect("invalid distance");

        if part2 {
            // Part 2: Count each click that lands on 0
            // Each rotation has 'dist' clicks
            // We need to count how many of those clicks land on 0
            
            for _ in 0..dist {
                match dir {
                    "L" => {
                        pos -= 1;
                        if pos < 0 {
                            pos = 99;
                        }
                    }
                    "R" => {
                        pos += 1;
                        if pos > 99 {
                            pos = 0;
                        }
                    }
                    _ => panic!("unknown direction: {dir}"),
                }
                
                if pos == 0 {
                    zero_hits += 1;
                }
            }
        } else {
            // Part 1: Only count when we end at 0
            match dir {
                "L" => pos -= dist,
                "R" => pos += dist,
                _ => panic!("unknown direction: {dir}"),
            }

            // wrap into 0..=99 (handle negatives correctly)
            pos = ((pos % 100) + 100) % 100;

            if pos == 0 {
                zero_hits += 1;
            }
        }
    }

    println!("Password: {}", zero_hits);
}
