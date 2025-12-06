#[derive(Debug, Clone)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug, Clone)]
struct Column {
    id: usize,
    value: Option<i128>,
    column_operator: Option<Operator>,
}

pub fn solve_part1() {
    let instructions =
        std::fs::read_to_string("input-aoc6.txt").expect("Failed to read input file");

    let mut columns: Vec<Column> = Vec::new();

    for line in instructions.trim_start().trim_end().lines() {
        let mut current_line_idx = 0;
        let parts: Vec<&str> = line.split_whitespace().collect();

        for part in parts {
            match part {
                "+" => {
                    columns.push(Column {
                        id: current_line_idx,
                        value: None,
                        column_operator: Some(Operator::Add),
                    });
                }
                "*" => {
                    columns.push(Column {
                        id: current_line_idx,
                        value: None,
                        column_operator: Some(Operator::Multiply),
                    });
                }
                _ => {
                    columns.push(Column {
                        id: current_line_idx,
                        value: Some(part.parse().unwrap_or(0)),
                        column_operator: None,
                    });
                }
            }

            current_line_idx += 1;
        }
    }

    let mut total: i128 = 0;

    loop {
        let mut col_result = 0;
        let mut col_id = 0;
        let max_col_id = columns.iter().map(|c| c.id).max().unwrap_or(0);

        while col_id <= max_col_id {
            col_result = evaluate_column(&columns, col_id);

            total += col_result;

            col_id += 1;
        }

        break;
    }

    println!("[PART 1]The total is: {}", total);
}

pub fn solve_part2() {
    let input = std::fs::read_to_string("input-aoc6.txt").expect("Failed to read input");

    // Parse input as a character grid, preserving exact positions
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        println!("[PART 2] Total = 0");
        return;
    }

    // Find max line length and pad all lines to same length
    let max_len = lines.iter().map(|l| l.len()).max().unwrap();
    let grid: Vec<Vec<char>> = lines
        .iter()
        .map(|l| {
            let mut chars: Vec<char> = l.chars().collect();
            chars.resize(max_len, ' ');
            chars
        })
        .collect();

    let num_rows = grid.len();
    let num_cols = max_len;
    let operator_row = num_rows - 1;

    let mut total: i128 = 0;
    let mut current_numbers: Vec<i128> = Vec::new();
    let mut current_op: Option<Operator> = None;

    for col in (0..num_cols).rev() {
        let is_blank = (0..num_rows).all(|row| grid[row][col] == ' ');

        if is_blank {
            if !current_numbers.is_empty() {
                let op = current_op.take().expect("Missing operator");
                let result: i128 = match op {
                    Operator::Add => current_numbers.iter().sum(),
                    Operator::Multiply => current_numbers.iter().product(),
                };
                total += result;
                current_numbers.clear();
            }
        } else {
            let bottom_char = grid[operator_row][col];
            if bottom_char == '+' {
                current_op = Some(Operator::Add);
            } else if bottom_char == '*' {
                current_op = Some(Operator::Multiply);
            }

            let mut digits: Vec<u32> = Vec::new();
            for row in 0..operator_row {
                let ch = grid[row][col];
                if let Some(d) = ch.to_digit(10) {
                    digits.push(d);
                }
            }

            if !digits.is_empty() {
                let mut value: i128 = 0;
                for d in digits {
                    value = value * 10 + d as i128;
                }
                current_numbers.push(value);
            }
        }
    }

    if !current_numbers.is_empty() {
        let op = current_op.take().expect("Missing operator");
        let result: i128 = match op {
            Operator::Add => current_numbers.iter().sum(),
            Operator::Multiply => current_numbers.iter().product(),
        };
        total += result;
    }

    println!("[PART 2] Total = {}", total);
}

fn evaluate_column(columns: &Vec<Column>, column_id: usize) -> i128 {
    let mut result: i128 = 0;

    let column = &columns
        .iter()
        .filter_map(|c| if c.id == column_id { Some(c) } else { None })
        .collect::<Vec<&Column>>();

    let col_operator = &column
        .iter()
        .filter_map(|c| c.column_operator.as_ref())
        .collect::<Vec<&Operator>>()[0];

    for col in column {
        let col_val = col.value.unwrap_or(0);

        if col.column_operator.is_none() {
            match col_operator {
                Operator::Add => {
                    result += col_val;
                }
                Operator::Multiply => {
                    if result == 0 {
                        result = 1; // to avoid multiplying by 0
                    }
                    result *= col_val;
                }
            }
        }
    }

    result
}
