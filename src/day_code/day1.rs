use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/*
fn read_text_file(fname: String) -> Result<(Vec<i32>, Vec<i32>), std::io::Error> {
    let mut first_vec: Vec<i32> = vec![];
    let mut second_vec: Vec<i32> = vec![];
    first_vec.reserve(1000);
    second_vec.reserve(1000);
    for line in reader.lines() {
        let line = line?;
        let nums: Vec<i32> = line
            .split_whitespace()
            .filter(|x| *x != "")
            .map(|x| x.parse::<i32>().expect("Not numbers"))
            .collect();
        first_vec.push(nums[0]);
        second_vec.push(nums[1]);
    }
    Ok((first_vec, second_vec))
}
*/
fn lr_to_num(s: String) -> Option<i32> {
    let (pre, num_str) = s.split_at_checked(1)?;
    let num: i32 = num_str.parse().ok()?;

    match pre {
        "L" => Some(-num),
        "R" => Some(num),
        _ => None,
    }
}

pub fn part_one(fname: String) -> Result<i32, std::io::Error> {
    let file = File::open(fname)?;
    let reader = BufReader::new(file);

    let values: Vec<_> = reader
        .lines()
        .filter_map(|s| lr_to_num(s.unwrap()))
        .collect();
    let mut start = 50;
    let mut counter = 0;
    for v in values {
        start = (start + v) % 100;
        if start == 0 {
            counter += 1;
        }
    }
    Ok(counter)
}

pub fn part_two(fname: String) -> Result<i32, std::io::Error> {
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
