use std::str::Lines;

pub fn day_one(input: String) -> u32 {
    let lines: Lines<'_> = input.lines();
    // Map lines to digits
    let calibration_values: Vec<u32> = lines
        .into_iter()
        .map(|f| get_line_calibration_value(f))
        .filter_map(|f| f.ok())
        .collect();

    calibration_values.into_iter().sum::<u32>()
}

fn get_line_calibration_value(line: &str) -> Result<u32, &str> {
    let digits: Vec<u32> = line
        .chars()
        .filter(|f| f.is_numeric())
        .filter_map(|f| f.to_digit(10))
        .collect();

    let first = digits.first();
    let last = digits.last();

    if first.is_none() || last.is_none() {
        return Err("No valid values found");
    }

    let first = first.unwrap();
    let last = last.unwrap();

    Ok((first * 10) + last)
}
