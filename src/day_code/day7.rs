use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn part_one(fname: String) -> Result<i64, std::io::Error> {
    let file = File::open(fname)?;
    let reader = BufReader::new(file);

    let mut lines: Vec<Vec<_>> = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| line.chars().collect())
        .collect();

    let mut counter = 0;
    for i in 1..lines.len() {
        for j in 0..lines[i].len() {
            if lines[i][j] == '^' && (lines[i - 1][j] == '|' || lines[i - 1][j] == 'S') {
                counter += 1;
                if j > 0 {
                    lines[i][j - 1] = '|';
                }
                if j < lines[i].len() - 1 {
                    lines[i][j + 1] = '|';
                }
            } else if lines[i - 1][j] == '|' || lines[i - 1][j] == 'S' {
                lines[i][j] = '|'
            }
        }
    }

    Ok(counter)
}

fn count_timelines(
    grid: &[Vec<char>],
    row: usize,
    col: usize,
    dp: &mut HashMap<(usize, usize), i64>,
) -> i64 {
    let rows = grid.len();

    if row >= rows {
        return 1;
    }
    if dp.contains_key(&(row, col)) {
        return dp[&(row, col)];
    }

    let ch = grid[row][col];
    let mut total = 0;

    if ch == '^' && row > 0 {
        if col > 0 {
            total += count_timelines(grid, row + 1, col - 1, dp);
        }
        if col < grid[0].len() - 1 {
            total += count_timelines(grid, row + 1, col + 1, dp);
        }
        dp.insert((row, col), total);
    } else {
        total += count_timelines(grid, row + 1, col, dp);
    }

    total
}

pub fn part_two(fname: String) -> Result<i64, std::io::Error> {
    let file = File::open(fname)?;
    let reader = BufReader::new(file);

    let grid: Vec<Vec<_>> = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| line.chars().collect())
        .collect();

    let start_col = grid[0]
        .iter()
        .position(|&ch| ch == 'S')
        .expect("No S in first row.");

    let mut dp_arr = HashMap::new();
    let timeline_count = count_timelines(&grid, 0, start_col, &mut dp_arr);

    Ok(timeline_count)
}
