use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

impl JunctionBox {
    fn new(x: i64, y: i64, z: i64) -> Self {
        JunctionBox { x, y, z }
    }

    fn squared_distance(&self, other: &JunctionBox) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

pub fn part_one(fname: String) -> Result<i64, std::io::Error> {
    let file = File::open(fname)?;
    let reader = BufReader::new(file);

    let boxes: Vec<JunctionBox> = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            let nums = line
                .split(',')
                .filter_map(|num_str| num_str.parse().ok())
                .collect::<Vec<i64>>();
            JunctionBox::new(nums[0], nums[1], nums[2])
        })
        .collect();

    let mut distances: Vec<Vec<i64>> = boxes
        .iter()
        .map(|box1| {
            boxes
                .iter()
                .map(|box2| {
                    if std::ptr::eq(box1, box2) {
                        i64::MAX
                    } else {
                        box1.squared_distance(box2)
                    }
                })
                .collect()
        })
        .collect();

    let mut circuits: HashMap<i64, i64> = HashMap::new();
    let mut num_circuits = 0;

    for _ in 0..1000 {
        let mut min_value = i64::MIN;
        let mut min_row = 0;
        let mut min_col = 0;

        for (row_idx, row) in distances.iter().enumerate() {
            for (col_idx, &value) in row.iter().enumerate() {
                if value > min_value && (!circuits.contains_key(row_idx) || !circuits.contains_key(col_idx)  circuits[row_idx]) {}
            }
        }
    }

    Ok(0)
}

pub fn part_two(fname: String) -> Result<i64, std::io::Error> {
    Ok(0)
}
