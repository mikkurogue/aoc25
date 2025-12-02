use std::error::Error;

const MOD: usize = 100;
struct Instruction(char, u32);

struct Lock {
    position: usize,
    instructions: Vec<Instruction>,
}

impl Lock {
    fn new(starting: usize, instructions: Vec<Instruction>) -> Self {
        Lock {
            position: starting,
            instructions,
        }
    }

    fn open_lock(&mut self) -> Result<usize, Box<dyn Error>> {
        let mut zero_count = 0;

        for ins in &self.instructions {
            zero_count += self.count_zeros(self.position, ins)?;
            self.position = self.final_position(self.position, ins)?;
        }

        Ok(zero_count)
    }

    fn count_zeros(&self, start: usize, ins: &Instruction) -> Result<usize, Box<dyn Error>> {
        let dist = ins.1 as usize;

        match ins.0 {
            'L' => {
                if dist == 0 {
                    Ok(0)
                } else if start == 0 {
                    Ok(dist / MOD)
                } else if dist >= start {
                    Ok(1 + ((dist - start) / MOD))
                } else {
                    Ok(0)
                }
            }
            'R' => {
                if dist == 0 {
                    Ok(0)
                } else if start == 0 {
                    Ok(dist / MOD)
                } else {
                    let steps_to_zero = MOD - start;
                    if dist >= steps_to_zero {
                        Ok(1 + ((dist - steps_to_zero) / MOD))
                    } else {
                        Ok(0)
                    }
                }
            }
            _ => Err(Box::from("Invalid direction")),
        }
    }

    fn final_position(&self, position: usize, ins: &Instruction) -> Result<usize, Box<dyn Error>> {
        match ins.0 {
            'L' => Ok(self.move_left(position, ins.1)),
            'R' => Ok(self.move_right(position, ins.1)),
            _ => Err(Box::from("Invalid direction")),
        }
    }

    fn move_left(&self, position: usize, distance: u32) -> usize {
        ((position + MOD) - (distance as usize % MOD)) % MOD as usize
    }

    fn move_right(&self, position: usize, distance: u32) -> usize {
        (position + (distance as usize % MOD)) % MOD as usize
    }
}

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

    let mut lock = Lock::new(starting, instructions);
    let zero_count = lock.open_lock()?;

    println!("Password: {}", zero_count);
    Ok(())
}

fn extract_instruction(instruction: &str) -> Instruction {
    let direction = instruction.chars().next().unwrap();
    let distance: u32 = instruction[1..].parse().unwrap();
    Instruction(direction, distance)
}
