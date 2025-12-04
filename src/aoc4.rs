enum CheckDirection {
    TopLeft,
    TopMiddle,
    TopRight,
    MiddleLeft,
    // center position is not a valid direction to check
    MiddleRight,
    BottomLeft,
    BottomMiddle,
    BottomRight,
}

impl CheckDirection {
    pub fn delta(self) -> (isize, isize) {
        match self {
            CheckDirection::TopLeft => (-1, -1),
            CheckDirection::TopMiddle => (-1, 0),
            CheckDirection::TopRight => (-1, 1),
            CheckDirection::MiddleLeft => (0, -1),
            CheckDirection::MiddleRight => (0, 1),
            CheckDirection::BottomLeft => (1, -1),
            CheckDirection::BottomMiddle => (1, 0),
            CheckDirection::BottomRight => (1, 1),
        }
    }
}

pub fn solve() {
    let instructions =
        std::fs::read_to_string("input-aoc4.txt").expect("Failed to read input file");

    let mut grid: Vec<Vec<char>> = instructions
        .trim_end()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let grid_height = grid.len();
    let grid_width = grid[0].len();

    let directions = [
        CheckDirection::TopLeft.delta(),
        CheckDirection::TopMiddle.delta(),
        CheckDirection::TopRight.delta(),
        CheckDirection::MiddleLeft.delta(),
        CheckDirection::MiddleRight.delta(),
        CheckDirection::BottomLeft.delta(),
        CheckDirection::BottomMiddle.delta(),
        CheckDirection::BottomRight.delta(),
    ];

    let mut accessible = 0;
    let mut removed_total = 0;

    loop {
        let mut to_remove = Vec::new();

        // flood out each accessible '@' roll from the grid, per iteration
        for y in 0..grid_height {
            for x in 0..grid_width {
                if grid[y][x] != '@' {
                    continue;
                }

                let mut count = 0;

                for (dy, dx) in &directions {
                    let neighbour_y = y as isize + dy;
                    let neighbour_x = x as isize + dx;

                    if neighbour_y >= 0
                        && neighbour_y < grid_height as isize
                        && neighbour_x >= 0
                        && neighbour_x < grid_width as isize
                    {
                        if grid[neighbour_y as usize][neighbour_x as usize] == '@' {
                            count += 1;
                        }
                    }
                }

                if count < 4 {
                    accessible += 1;
                    to_remove.push((y, x));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        for (y, x) in to_remove.iter() {
            grid[*y][*x] = '.';
        }

        removed_total += to_remove.len();
    }

    println!("Accessible rolls: {}", accessible);
    println!("Total removed: {}", removed_total);
}
