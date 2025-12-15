use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn find_best_voltage(v: Vec<i64>) -> i64 {
    let (idx, best) = v[..v.len() - 1]
        .iter()
        .enumerate()
        .max_by_key(|&(i, &val)| (val, std::cmp::Reverse(i)))
        .expect("No max?");
    let second_best = v[(idx + 1)..].iter().max().expect("No max?");

    best * 10 + second_best
}

pub fn part_one(fname: String) -> Result<i64, std::io::Error> {
    let file = File::open(fname)?;
    let reader = BufReader::new(file);

    let values: i64 = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10).unwrap() as i64)
                .collect()
        })
        .map(find_best_voltage)
        .sum();

    Ok(values)
}

fn find_best_voltage_pt2(v: Vec<i64>) -> i64 {
    // Joltage length
    let jl = 12;
    /*
    let mut result = 0;
    let mut current_pos = 0;
    for i in (0..jl).rev() {
        let (offset, best) = v[current_pos..v.len() - i]
            .iter()
            .enumerate()
            .max_by_key(|&(i, &val)| (val, std::cmp::Reverse(i)))
            .expect("No max?");

        result = result * 10 + best;
        current_pos += offset + 1;
    }

    result
    */

    // crazy fold with the help of AI
    (0..jl)
        .rev()
        .fold((0, 0), |(acc, start), remaining_len| {
            let (offset, &digit) = v[start..(v.len() - remaining_len)]
                .iter()
                .enumerate()
                .rev()
                .max_by_key(|&(i, val)| (val, std::cmp::Reverse(i)))
                .unwrap();

            (acc * 10 + digit, start + offset + 1)
        })
        .0
}

pub fn part_two(fname: String) -> Result<i64, std::io::Error> {
    let file = File::open(fname)?;
    let reader = BufReader::new(file);

    let values: i64 = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10).unwrap() as i64)
                .collect()
        })
        .map(find_best_voltage_pt2)
        .sum();

    Ok(values)
}
