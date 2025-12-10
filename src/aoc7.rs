#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TeleporterTokens {
    EntryPoint,
    EmptySpace,
    Splitter,
}

impl TeleporterTokens {
    pub fn from_char(c: char) -> Self {
        match c {
            'S' => TeleporterTokens::EntryPoint,
            '.' => TeleporterTokens::EmptySpace,
            '^' => TeleporterTokens::Splitter,
            _ => panic!("Unknown token character: {}", c),
        }
    }
}

#[derive(Debug)]
struct TokenPosition {
    line_no: usize,
    col_no: usize,
    token: TeleporterTokens,
    visited: bool,
}

pub fn solve_part1() {
    let input = std::fs::read_to_string("input-aoc7.txt").expect("Failed to read input file");
    let input = input.trim(); // trim the input just incase

    let mut grid_w = 0;

    let mut line_no = 0;

    let mut grid_tokens: Vec<TokenPosition> = Vec::new();

    for line in input.lines() {
        line_no += 1;
        grid_w = grid_w.max(line.len());

        for (col_no, c) in line.chars().enumerate() {
            let token = TeleporterTokens::from_char(c);
            grid_tokens.push(TokenPosition {
                line_no,
                col_no: col_no + 1,
                token,
                visited: false,
            });
        }
    }

    let split_count = traverse_grid(&mut grid_tokens);

    println!("Number of splitters encountered: {}", split_count);

    // println!("Grid dimensions: width = {}, height = {}", grid_w, grid_h);

    // println!("Parsed tokens: {:#?}", grid_tokens);
}

/// Part 1 traversal to count amount of splits
fn traverse_grid(grid_tokens: &mut Vec<TokenPosition>) -> usize {
    let mut beams: Vec<(usize, usize)> = Vec::new();
    let mut split_count = 0;

    // find entry
    for cell in grid_tokens.iter() {
        if cell.token == TeleporterTokens::EntryPoint {
            beams.push((cell.line_no, cell.col_no));
            break;
        }
    }

    while let Some((row, col)) = beams.pop() {
        let cell_opt = grid_tokens
            .iter_mut()
            .find(|x| x.line_no == row && x.col_no == col);

        let Some(cell) = cell_opt else {
            continue;
        };

        if cell.visited {
            continue;
        }
        cell.visited = true;

        match cell.token {
            TeleporterTokens::EmptySpace | TeleporterTokens::EntryPoint => {
                // continue down
                beams.push((row + 1, col));
            }
            TeleporterTokens::Splitter => {
                split_count += 1;

                if col > 1 {
                    beams.push((row, col - 1));
                }
                beams.push((row, col + 1));
            }
        }
    }

    split_count
}
