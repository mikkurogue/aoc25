pub fn solve() {
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
