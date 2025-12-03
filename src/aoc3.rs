// solve part 2
pub fn solve() {
    let instructions =
        std::fs::read_to_string("input-aoc3.txt").expect("Failed to read input file");

    let mut max_joltages_per_line: Vec<u64> = vec![];

    let keep = 12usize;

    for line in instructions.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let digits: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let line_len = digits.len();

        let mut remove = line_len - keep;
        // use a monotonic decreasing stack to find the largest possible number without reordering
        let mut stack: Vec<u32> = Vec::with_capacity(line_len);

        for digit in digits {
            while remove > 0 && !stack.is_empty() && stack.last().unwrap() < &digit {
                stack.pop();
                remove -= 1;
            }
            stack.push(digit);
        }

        while remove > 0 {
            stack.pop();
            remove -= 1;
        }

        stack.truncate(keep);

        let val = stack.iter().fold(0u64, |acc, &x| acc * 10 + x as u64);

        max_joltages_per_line.push(val);
    }

    let total: u128 = max_joltages_per_line.iter().map(|&v| v as u128).sum();

    println!("Total maximum joltage: {}", total);
}

// solve part 1
fn _solve() {
    let instructions =
        std::fs::read_to_string("input-aoc3.txt").expect("Failed to read input file");

    let mut max_joltage = 0;
    let mut max_joltages_per_line: Vec<u32> = vec![];

    for line in instructions.lines() {
        // println!("Processing line: {}", line);

        let joltage: String = line.trim().parse().expect("Failed to parse joltage");

        let digits: Vec<u32> = joltage.chars().map(|c| c.to_digit(10).unwrap()).collect();

        let mut max_pair = 0;

        for i in 0..digits.len() - 1 {
            for j in (i + 1)..digits.len() {
                let a = digits[i];
                let b = digits[j];

                let pair = a * 10 + b;
                if pair > max_pair {
                    max_pair = pair;
                }
            }
        }

        // push the max pair to the vector when iteration is done
        max_joltages_per_line.push(max_pair);
    }

    max_joltage = max_joltages_per_line.iter().sum();

    println!("Total maximum joltage: {}", max_joltage);
}
