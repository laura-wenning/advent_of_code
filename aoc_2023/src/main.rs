use std::fs;

use day_1::day_one;

mod day_1;

fn main() {
    let input = fs::read_to_string("./inputs/day_one.txt").expect("Unable to read 'day_one.txt'.");
    let res = day_one(input);
    println!("Result: {}", res);
}
