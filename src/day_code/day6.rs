use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn part_one(fname: String) -> Result<i64, std::io::Error> {
    let file = File::open(fname)?;
    let reader = BufReader::new(file);

    let lines: Vec<_> = reader.lines().map_while(Result::ok).collect();

    let values: Vec<Vec<_>> = lines
        .iter()
        .take(4)
        .map(|line| {
            line.split_whitespace()
                .filter_map(|number| number.parse::<i64>().ok())
                .collect()
        })
        .collect();

    let res = lines
        .last()
        .unwrap()
        .split_whitespace()
        .enumerate()
        .map(|(i, c)| match c {
            "*" => values[0][i] * values[1][i] * values[2][i] * values[3][i],
            _ => values[0][i] + values[1][i] + values[2][i] + values[3][i],
        })
        .sum();

    Ok(res)
}

enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Operation::Add => a + b,
            Operation::Multiply => a * b,
        }
    }
}

pub fn part_two(fname: String) -> Result<i64, std::io::Error> {
    let file = File::open(fname)?;
    let reader = BufReader::new(file);

    let lines: Vec<_> = reader.lines().map_while(Result::ok).collect();

    let values: Vec<Vec<_>> = lines
        .iter()
        .take(4)
        .map(|line| line.chars().collect())
        .collect();

    let mut iter = lines[4].chars().enumerate().peekable();
    let mut res = 0;

    while let Some((i, op_char)) = iter.next() {
        let op = match op_char {
            '*' => Operation::Multiply,
            '+' => Operation::Add,
            _ => panic!("Invalid operator: {}", op_char),
        };
        let mut len = 0;
        while let Some(&(_, ' ')) = iter.peek() {
            iter.next();
            len += 1;
        }
        if iter.peek() == None {
            len += 1;
        }

        let mut acc = match op {
            Operation::Add => 0,
            Operation::Multiply => 1,
        };
        for j in 0..len {
            let mut num_str: String = String::new();
            for k in 0..3 {
                let ch = values[k][i + j];
                if ch.is_numeric() {
                    num_str.push(ch);
                }
            }

            let num: i64 = num_str
                .parse::<i64>()
                .expect(format!("value: {}, i: {}, j: {}", num_str, i, j).as_str());
            acc = op.apply(acc, num);
        }
        res += acc;
    }

    Ok(res)
}
