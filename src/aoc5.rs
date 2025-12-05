#[derive(Debug)]
struct Range {
    start: i64,
    end: i64,
}

pub fn solve_part2() {
    let instructions =
        std::fs::read_to_string("input-aoc5.txt").expect("Failed to read input file");

    let mut ranges: Vec<Range> = Vec::new();

    instructions.trim_end().lines().for_each(|line| {
        // Skip empty lines
        if line.is_empty() {
            return;
        }

        if line.contains('-') {
            let parts: Vec<&str> = line.split('-').collect();
            if parts.len() != 2 {
                eprintln!("Invalid range format: {}", line);
                return;
            }

            let start = parts[0].parse::<i64>();
            let end = parts[1].parse::<i64>();

            match (start, end) {
                (Ok(s), Ok(e)) => {
                    ranges.push(Range { start: s, end: e });
                }
                _ => {
                    eprintln!("Failed to parse range: {}", line);
                }
            }

            return;
        }
    });

    ranges.sort_by_key(|r| r.start);

    let mut merged_ranges: Vec<Range> = Vec::new();

    for range in ranges {
        if let Some(last) = merged_ranges.last_mut() {
            if range.start <= last.end + 1 {
                last.end = last.end.max(range.end);
            } else {
                merged_ranges.push(range);
            }
        } else {
            merged_ranges.push(range);
        }
    }

    let total = merged_ranges
        .iter()
        .map(|r| r.end - r.start + 1)
        .sum::<i64>();

    println!("Unique fresh ingredients: {:#?}", total);
}

pub fn solve_part1() {
    let instructions =
        std::fs::read_to_string("input-aoc5.txt").expect("Failed to read input file");

    let mut ranges: Vec<Range> = Vec::new();
    let mut values_to_check: Vec<i64> = Vec::new();

    instructions.trim_end().lines().for_each(|line| {
        // Skip empty lines
        if line.is_empty() {
            return;
        }

        if line.contains('-') {
            let parts: Vec<&str> = line.split('-').collect();
            if parts.len() != 2 {
                eprintln!("Invalid range format: {}", line);
                return;
            }

            let start = parts[0].parse::<i64>();
            let end = parts[1].parse::<i64>();

            match (start, end) {
                (Ok(s), Ok(e)) => {
                    ranges.push(Range { start: s, end: e });
                }
                _ => {
                    eprintln!("Failed to parse range: {}", line);
                }
            }

            return;
        }

        // the values we want to check
        if !line.contains('-') {
            line.parse::<i64>()
                .map(|value| values_to_check.push(value))
                .unwrap_or_else(|_| {
                    eprintln!("Failed to parse value to check: {}", line);
                });

            return;
        }
    });

    let mut found_count = 0;

    for value in &values_to_check {
        let mut found = false;
        for range in &ranges {
            if is_value_in_range(*value, range) {
                println!("Value {} is in range {:?}.", value, range);
                found = true;
                found_count += 1;
                break;
            }
        }
        if !found {
            println!("Value {} is NOT in any range.", value);
        }
    }

    // println!("Ranges: {:#?}", ranges);
    // println!("Values to check: {:#?}", values_to_check);

    println!("Fresh ingredients: {}", found_count);
}

fn is_value_in_range(value: i64, range: &Range) -> bool {
    value >= range.start && value <= range.end
}
