use std::fs;

// use day_1::day_one;
use day_2::day_two_part_two;

// mod day_1;
mod day_2;

fn main() {
    let target = "day02";
    let path = format!("./inputs/{}.txt", target);
    let input =
        fs::read_to_string(path).unwrap_or_else(|_| panic!("Unable to read '{}.txt'.", target));
    let res = day_two_part_two(input);
    println!("Result: {}", res);
}
