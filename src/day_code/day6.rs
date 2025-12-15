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

    Ok(0)
}

pub fn part_two(fname: String) -> Result<i64, std::io::Error> {
    Ok(0)
}
