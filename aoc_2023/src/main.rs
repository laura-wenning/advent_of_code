use std::fs;

use day_1::day_one;

mod day_1;

fn main() {
    let target = "day01";
    let path = format!("./inputs/{}.txt", target);
    let input =
        fs::read_to_string(path).unwrap_or_else(|_| panic!("Unable to read '{}.txt'.", target));
    let res = day_one(input);
    println!("Result: {}", res);
}
