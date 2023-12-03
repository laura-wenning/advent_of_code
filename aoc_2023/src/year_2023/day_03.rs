use regex::Regex;

pub fn day_03_part_1(input: String) -> u32 {
    let engine = GondolaEngine::new(input);
    let possible_parts = engine.identify_possible_parts();
    let valid_parts: Vec<Part> = possible_parts
        .into_iter()
        .filter(|part| engine.is_valid_part(part))
        .collect();

    let part_number_sum: i32 = valid_parts
        .iter()
        .map(|part| {
            println!("{}", part.part_number);
            part.part_number
        })
        .sum();
    part_number_sum.try_into().unwrap()
}

#[derive(Clone)]
struct GondolaEngine {
    schematic: String,
    raw_schematic: Vec<Vec<char>>,
}

impl GondolaEngine {
    pub fn new(schematic: String) -> Self {
        Self {
            schematic: schematic.clone(),
            raw_schematic: Self::to_raw_schematic(schematic),
        }
    }

    fn to_raw_schematic(schematic: String) -> Vec<Vec<char>> {
        let raw_schematic: Vec<Vec<char>> = schematic
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        raw_schematic
    }

    pub fn identify_possible_parts(&self) -> Vec<Part> {
        let part_number_regex = Regex::new(r"[0-9]+").unwrap();
        let schematic_lines = self.schematic.lines();

        let mut possible_parts: Vec<Part> = vec![];

        for (line_number, line) in schematic_lines.enumerate() {
            let mut line_parts =
                self.identify_possible_parts_in_line(line_number, line, &part_number_regex);
            possible_parts.append(&mut line_parts);
        }

        possible_parts
    }

    fn identify_possible_parts_in_line(
        &self,
        line_number: usize,
        line: &str,
        regex: &Regex,
    ) -> Vec<Part> {
        let matches = regex.find_iter(line);
        let parts: Vec<Part> = matches
            .map(|part_match| {
                Part::new(
                    part_match.as_str().parse().unwrap(),
                    line_number,
                    part_match.start(),
                    part_match.end(),
                )
            })
            .collect();
        parts
    }

    pub fn is_valid_part(&self, possible_part: &Part) -> bool {
        let x_bounds = possible_part.get_x_bounds(self.get_schematic_width());
        let y_bounds = possible_part.get_y_bounds(self.get_schematic_height());
        println!(
            "Part: {}, {:?}, {:?}",
            possible_part.part_number, x_bounds, y_bounds
        );
        // panic!();
        for y_position in y_bounds.0..=y_bounds.1 {
            for x_position in x_bounds.0..=x_bounds.1 {
                let possible_symbol = match self.get_character_at(x_position, y_position) {
                    None => continue,
                    Some(x) => x,
                };
                if is_symbol(possible_symbol) {
                    return true;
                }
            }
        }
        false
    }

    fn get_schematic_height(&self) -> usize {
        self.raw_schematic.len()
    }

    fn get_schematic_width(&self) -> usize {
        self.raw_schematic.get(0).unwrap().len()
    }

    fn get_character_at(&self, x: usize, y: usize) -> Option<char> {
        Some(*self.raw_schematic.get(y)?.get(x)?)
    }
}

fn is_symbol(possible_symbol: char) -> bool {
    if possible_symbol == '.' {
        return false;
    }
    if possible_symbol.is_ascii_digit() {
        return false;
    }
    true
}

#[derive(Debug)]
struct Part {
    part_number: i32,
    line: usize,
    start: usize,
    end: usize,
}

impl Part {
    pub fn new(part_number: i32, line: usize, start: usize, end: usize) -> Self {
        Self {
            part_number,
            line,
            start,
            end,
        }
    }

    pub fn get_x_bounds(&self, maximum_bound: usize) -> (usize, usize) {
        let x_lesser_bound = if self.start == 0 { 0 } else { self.start - 1 };
        let x_greater_bound = (self.end).min(maximum_bound);
        (x_lesser_bound, x_greater_bound)
    }

    pub fn get_y_bounds(&self, maximum_bound: usize) -> (usize, usize) {
        let y_lesser_bound = if self.line == 0 { 0 } else { self.line - 1 };
        let y_greater_bound = (self.line + 1).min(maximum_bound);
        (y_lesser_bound, y_greater_bound)
    }
}
