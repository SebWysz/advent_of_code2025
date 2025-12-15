use interavl::IntervalTree;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn part_one(fname: String) -> Result<i64, std::io::Error> {
    let file = File::open(fname)?;
    let reader = BufReader::new(file);

    let mut tree = IntervalTree::default();

    let lines: Vec<_> = reader.lines().map_while(Result::ok).collect();

    lines
        .iter()
        .take_while(|line| !line.is_empty())
        .map(|line| line.split_once('-').unwrap())
        .for_each(|(start, end)| {
            tree.insert(
                start.parse::<i64>().unwrap()..(end.parse::<i64>().unwrap() + 1),
                1,
            );
        });

    // Skip blank line
    let blank_idx = lines.iter().position(|line| line.is_empty()).unwrap_or(0);

    let res = lines[blank_idx + 1..]
        .iter()
        .filter_map(|line| line.parse::<i64>().ok())
        .filter(|num| tree.iter_overlaps(&(*num..(num + 1))).next().is_some())
        .count() as i64;

    Ok(res)
}

pub fn part_two(fname: String) -> Result<i64, std::io::Error> {
    let file = File::open(fname)?;
    let reader = BufReader::new(file);

    let mut intervals: Vec<_> = reader
        .lines()
        .map_while(Result::ok)
        .take_while(|line| !line.is_empty())
        .filter_map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            Some((start.parse::<i64>().ok()?, end.parse::<i64>().ok()?))
        })
        .collect();

    intervals.sort_by_key(|&(start, _)| start);

    let mut merged: Vec<(i64, i64)> = Vec::new();
    intervals.into_iter().for_each(|(start, end)| {
        if let Some(last) = merged.last_mut() {
            if start <= last.1 {
                last.1 = last.1.max(end);
            } else {
                merged.push((start, end))
            }
        } else {
            merged.push((start, end))
        }
    });

    let res = merged.iter().map(|(start, end)| end - start + 1).sum();

    Ok(res)
}
