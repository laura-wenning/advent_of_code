use advent::AdventOfCode;

mod year_2023;

pub mod advent;
pub mod challenge;
pub mod config;

fn main() {
    let advent = AdventOfCode::new();
    let result = advent.run();
    if let Err(why) = result {
        println!("Failed: {}", why);
    }
}
