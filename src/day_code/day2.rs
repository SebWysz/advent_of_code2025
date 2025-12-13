use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn is_invalid_id(n: &i64) -> bool {
    let s = n.to_string();

    if s.len() % 2 != 0 {
        return false;
    }

    let (first, second) = s.split_at(s.len() / 2);
    first == second
}

pub fn part_one(fname: String) -> Result<i64, std::io::Error> {
    let file = File::open(fname)?;
    let reader = BufReader::new(file);

    let line: String = reader.lines().next().ok_or("").unwrap().unwrap();
    let res = line
        .split(',')
        .map(|s| {
            let (one, two) = s.split_once('-').unwrap();
            (one.parse().unwrap(), two.parse().unwrap())
        })
        .map(|(start, end): (i64, i64)| (start..=end).filter(|&n| is_invalid_id(&n)))
        .flatten()
        .sum();

    Ok(res)
}

fn is_invalid_id_pt2(n: &i64) -> bool {
    let s = n.to_string();
    let len = s.len();

    /*
    let mid = len / 2;
    for sl in 1..=mid {
        if len % sl != 0 {
            continue;
        }
        let mut good = true;
        for i in 0..(len / sl - 1) {
            if s[(i * sl)..((i + 1) * sl)] != s[((i + 1) * sl)..((i + 2) * sl)] {
                good = false;
                break;
            }
        }
        if good {
            return true;
        }
    }

    false
    */

    let bytes = s.as_bytes();

    (1..=len / 2).any(|sl| len % sl == 0 && bytes.chunks(sl).all(|chunk| chunk == &bytes[..sl]))
}

pub fn part_two(fname: String) -> Result<i64, std::io::Error> {
    let file = File::open(fname)?;
    let reader = BufReader::new(file);

    let line: String = reader.lines().next().ok_or("").unwrap().unwrap();
    let res = line
        .split(',')
        .map(|s| {
            let (one, two) = s.split_once('-').unwrap();
            (one.parse().unwrap(), two.parse().unwrap())
        })
        .map(|(start, end): (i64, i64)| (start..=end).filter(|&n| is_invalid_id_pt2(&n)))
        .flatten()
        .sum();

    Ok(res)
}
