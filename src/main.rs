pub mod day_code;

use day_code::day1;

fn main() {
    // let res = day1::first();
    let res = day1::second();
    match res {
        Err(e) => println!("Error: {}", e),
        Ok(n) => println!("Success! Result: {}", n),
    }
}
