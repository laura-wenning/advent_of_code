use crate::advent::AdventOfCode;

mod day_01;
mod day_02;

pub fn register(advent: &mut AdventOfCode) {
    let year: u32 = 2023;
    advent.register(year, 1, 1, day_01::day_one);
    advent.register(year, 2, 1, day_02::day_two);
    advent.register(year, 2, 2, day_02::day_two_part_two);
    // day_01::register(advent_of_code);
}
