use ndarray::{Array2, s};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn part_one(fname: String) -> Result<i64, std::io::Error> {
    let file = File::open(fname)?;
    let reader = BufReader::new(file);

    let values: Vec<Vec<_>> = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '@' => 1,
                    _ => 0,
                })
                .into_iter()
                .collect()
        })
        .collect();
    let rows = values.len();
    let cols = values[0].len();

    let matrix = Array2::from_shape_vec(
        (rows, cols),
        values.into_iter().flatten().collect::<Vec<i64>>(),
    )
    .unwrap();

    let mut padded = Array2::from_elem((rows + 2, cols + 2), 0);
    padded.slice_mut(s![1..=rows, 1..=cols]).assign(&matrix);

    let res = padded
        .windows((3, 3))
        .into_iter()
        .filter(|window| window[[1, 1]] == 1 && window.sum() < 5)
        .count() as i64;

    Ok(res)
}

pub fn part_two(fname: String) -> Result<i64, std::io::Error> {
    let file = File::open(fname)?;
    let reader = BufReader::new(file);

    let values: Vec<Vec<_>> = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '@' => 1,
                    _ => 0,
                })
                .into_iter()
                .collect()
        })
        .collect();
    let rows = values.len();
    let cols = values[0].len();

    let matrix = Array2::from_shape_vec(
        (rows, cols),
        values.into_iter().flatten().collect::<Vec<i64>>(),
    )
    .unwrap();

    let mut padded = Array2::from_elem((rows + 2, cols + 2), 0);
    padded.slice_mut(s![1..=rows, 1..=cols]).assign(&matrix);

    let mut old_counter = -1;
    let mut counter = 0;
    while old_counter != counter {
        old_counter = counter;

        for i in 1..=rows {
            for j in 1..=cols {
                if padded[[i, j]] == 1
                    && padded[[i - 1, j - 1]]
                        + padded[[i, j - 1]]
                        + padded[[i + 1, j - 1]]
                        + padded[[i + 1, j]]
                        + padded[[i + 1, j + 1]]
                        + padded[[i, j + 1]]
                        + padded[[i - 1, j + 1]]
                        + padded[[i - 1, j]]
                        < 4
                {
                    counter += 1;
                    padded[[i, j]] = 0;
                }
            }
        }
    }
    Ok(counter)
}
