use std::str::Lines;

use regex::Regex;

use crate::advent::AdventOfCode;

struct WordMapping {
    text: &'static str,
    value: char,
}

impl WordMapping {
    pub fn new(text: &'static str, value: char) -> Self {
        Self { text, value }
    }
}

pub fn register(advent: &mut AdventOfCode) {
    let year = 2023;
    let day = 1;
    advent.register(year, day, 1, day_one);
    advent.register(year, day, 2, day_one);
}

pub fn day_one(input: String) -> u32 {
    let lines: Lines<'_> = input.lines();

    // Map lines to digits
    let calibration_values: Vec<u32> = lines
        .into_iter()
        .filter_map(calculate_calibration_value)
        .collect();
    println!("{:?}", calibration_values.len());
    calibration_values.into_iter().sum::<u32>()
}

fn get_word_mapping() -> Vec<WordMapping> {
    let word_map: Vec<WordMapping> = vec![
        WordMapping::new("one", '1'),
        WordMapping::new("two", '2'),
        WordMapping::new("three", '3'),
        WordMapping::new("four", '4'),
        WordMapping::new("five", '5'),
        WordMapping::new("six", '6'),
        WordMapping::new("seven", '7'),
        WordMapping::new("eight", '8'),
        WordMapping::new("nine", '9'),
    ];
    word_map
}

fn calculate_calibration_value(line: &str) -> Option<u32> {
    // !TODO - there is a bug with this where a value like "twone" registers as "two" and not one, giving us incorrect results
    // ! This seems to be something consistent with Regex in general, so regex as a solution is not viable
    let number_regex = Regex::new(r"[1-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let line = line.to_lowercase();

    let mut matches = number_regex.find_iter(line.as_str());

    let first_match = matches.next()?;
    let last_match = matches.last().unwrap_or(first_match);

    let first = get_first_number(first_match.as_str().to_string())?;
    let last = get_first_number(last_match.as_str().to_string())?;

    // let first_match = number_regex.find(line.as_str());
    // let first_match = number_regex.find(line.as_str());

    // let first_index = line.find_(number_regex)?;
    // let last_index = line.rfind(number_regex)?;

    // let first_substr = line.clone().split_off(first_index);
    // // TODO - check if they're the same index to cut off some work
    // let last_substr = line.clone().split_off(last_index);

    // let first = get_first_number(first_substr)?;
    // let last = get_first_number(last_substr)?;

    Some((first * 10) + last)
}

fn get_first_number(line: String) -> Option<u32> {
    if line.is_empty() {
        return None;
    }

    let first_character = line.chars().next()?;
    if first_character.is_numeric() {
        return first_character.to_digit(10);
    }

    let word_mappings = get_word_mapping();
    for mapping in word_mappings.iter() {
        if line.find(mapping.text) != Some(0) {
            continue;
        }

        let value = mapping.value.to_digit(10)?;
        return Some(value);
    }
    None
}
