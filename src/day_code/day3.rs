use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn part_one(fname: String) -> Result<i64, std::io::Error> {
    Ok(0)
}

pub fn part_two(fname: String) -> Result<i64, std::io::Error> {
    let file = File::open(fname)?;
    let reader = BufReader::new(file);

    let values: Vec<_> = reader
        .lines()
        .filter_map(|s| lr_to_num(s.unwrap()))
        .collect();
    let mut start = 50;
    let mut counter = 0;
    for v in values {
        let new = start + v;
        counter += new.abs() / 100;
        if start.signum() != 0 && new.signum() != start.signum() {
            counter += 1;
        }
        start = new % 100;
    }
    Ok(counter)
}
