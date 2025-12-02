#[derive(Debug)]
struct InstructionRange(i64, i64);

struct IdChecker {
    range: Vec<InstructionRange>,
}

impl IdChecker {
    fn new(range: Vec<InstructionRange>) -> Self {
        IdChecker { range }
    }

    fn check_id(&self, n: i64) -> bool {
        let s = n.to_string();
        if s.starts_with('0') {
            return false;
        }

        let len = s.len();
        for b in 1..=len / 2 {
            if len % b != 0 {
                continue;
            }

            let pattern = &s[..b];
            if pattern.repeat(len / b) == s {
                return true;
            }
        }

        false
    }

    fn find_invalid_ids(&self, r: &InstructionRange) -> Vec<i64> {
        (r.0..=r.1).filter(|n| self.check_id(*n)).collect()
    }

    fn sum_invalid_ids(&mut self) -> i64 {
        self.range
            .iter()
            .flat_map(|r| self.find_invalid_ids(r))
            .map(|x| x as i64)
            .sum()
    }
}

pub fn solve() {
    let instructions = read_instruction();

    let mut checker = IdChecker::new(instructions);
    let total_sum = checker.sum_invalid_ids();
    println!("Total Sum of Valid IDs: {}", total_sum);
}

fn read_instruction() -> Vec<InstructionRange> {
    let input = std::fs::read_to_string("input-aoc2.txt").expect("Failed to read file");

    input
        .split(',')
        .filter(|x| !x.trim().is_empty())
        .map(|chunk| {
            let (a, b) = chunk.trim().split_once('-').expect("Failed to split range");

            InstructionRange(a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap())
        })
        .collect()
}
