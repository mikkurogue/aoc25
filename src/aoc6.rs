#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug)]
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

    println!("The total is: {}", total);
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
