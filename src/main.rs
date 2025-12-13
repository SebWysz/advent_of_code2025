pub mod day_code;

use clap::{Parser, ValueEnum};
use day_code::{day1, day2, day3};
use std::io::{Error, ErrorKind};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
enum Part {
    One,
    Two,
}

#[derive(Parser, Debug)]
struct Cli {
    // Day number, e.g. --day 5
    #[arg(long)]
    day: u32,

    // Which part to run
    #[arg(value_enum, short, long, default_value_t = Part::One)]
    part: Part,
}

fn main() {
    let cli = Cli::parse();
    let file_name = format!("./textfiles/day{}.txt", cli.day);

    let res = match (cli.day, cli.part) {
        (1, Part::One) => day1::part_one(file_name),
        (1, Part::Two) => day1::part_two(file_name),
        (2, Part::One) => day2::part_one(file_name),
        (2, Part::Two) => day2::part_two(file_name),
        (3, Part::One) => day3::part_one(file_name),
        (3, Part::Two) => day3::part_two(file_name),
        _ => Err(Error::new(ErrorKind::Other, "Invalid CLI Args")),
    };

    match res {
        Err(e) => println!("Error: {}", e),
        Ok(n) => println!("Success! Result: {}", n),
    }
}
