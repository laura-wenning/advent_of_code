use crate::advent::AdventOfCode;

mod day_01;
mod day_02;
mod day_03;

pub fn register(advent: &mut AdventOfCode) {
    let year: u32 = 2023;
    advent.register(year, 1, 1, day_01::day_one);
    advent.register(year, 2, 1, day_02::day_two);
    advent.register(year, 2, 2, day_02::day_two_part_two);
    advent.register(year, 3, 1, day_03::day_03_part_1);
    advent.register(year, 3, 2, day_03::day_03_part_2);
}
