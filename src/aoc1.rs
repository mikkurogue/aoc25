use std::error::Error;

fn read_instructions() -> Vec<Instruction> {
    let read_file = std::fs::read_to_string("input-aoc1.txt").expect("Failed to read file");

    let lines: Vec<Instruction> = read_file
        .trim_end()
        .lines()
        .map(|line| extract_instruction(line))
        .collect();

    lines
}

pub fn solve() -> Result<(), Box<dyn Error>> {
    let starting = 50usize;
    let instructions = read_instructions();

    let mut position: usize = starting;
    let mut zero_count = 0;

    for ins in instructions {
        zero_count += count_zeros(position, &ins)?;
        position = final_position(position, &ins)?;
    }

    println!("Password: {}", zero_count);
    Ok(())
}

struct Instruction(char, u32);

fn extract_instruction(instruction: &str) -> Instruction {
    let direction = instruction.chars().next().unwrap();
    let distance: u32 = instruction[1..].parse().unwrap();
    Instruction(direction, distance)
}

const MOD: usize = 100;

fn move_left(position: usize, distance: u32) -> usize {
    ((position + MOD) - (distance as usize % MOD)) % MOD as usize
}

fn move_right(position: usize, distance: u32) -> usize {
    (position + (distance as usize % MOD)) % MOD as usize
}

fn count_zeros(start: usize, ins: &Instruction) -> Result<usize, Box<dyn Error>> {
    let dist = ins.1 as usize;

    match ins.0 {
        'L' => {
            if dist >= start {
                Ok(1 + ((dist - start) / 100))
            } else {
                Ok(0)
            }
        }
        'R' => {
            let first = (100 - start) % 100;
            if dist >= first {
                Ok(1 + ((dist - first) / 100))
            } else {
                Ok(0)
            }
        }
        _ => Err(Box::from("Invalid direction")),
    }
}

fn final_position(position: usize, ins: &Instruction) -> Result<usize, Box<dyn Error>> {
    match ins.0 {
        'L' => Ok(move_left(position, ins.1)),
        'R' => Ok(move_right(position, ins.1)),
        _ => Err(Box::from("Invalid direction")),
    }
}
