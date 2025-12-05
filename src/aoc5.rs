#[derive(Debug)]
struct Range {
    start: i64,
    end: i64,
}

pub fn solve_part2() {
    let mut ranges: Vec<Range> = parse_range();

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
fn parse_range() -> Vec<Range> {
    std::fs::read_to_string("input-aoc5.txt")
        .expect("Failed to read input file")
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.contains('-') { Some(line) } else { None }
        })
        .map(|line| {
            let (s, e) = line.split_once('-').unwrap();
            Range {
                start: s.parse().unwrap(),
                end: e.parse().unwrap(),
            }
        })
        .collect()
}

fn parse_values() -> Vec<i64> {
    std::fs::read_to_string("input-aoc5.txt")
        .expect("Failed to read input file")
        .lines()
        .map(|l| l.trim())
        .filter_map(|line| line.parse::<i64>().ok())
        .collect()
}

pub fn solve_part1() {
    let ranges: Vec<Range> = parse_range();
    let values_to_check: Vec<i64> = parse_values();
    let mut found_count = 0;

    values_to_check.iter().for_each(|v| {
        let mut found = false;
        ranges.iter().for_each(|r| {
            if is_value_in_range(*v, r) && !found {
                println!("Value {} is in range {:?}.", v, r);
                found = true;
                found_count += 1;
            }
        });

        if !found {
            println!("Value {} is NOT in any range.", v);
        }
    });

    println!("Fresh ingredients: {}", found_count);
}

fn is_value_in_range(value: i64, range: &Range) -> bool {
    value >= range.start && value <= range.end
}
