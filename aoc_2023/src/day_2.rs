use regex::{Match, Regex};

struct Game {
    id: u32,
    results: Vec<GameResult>,
}

impl Game {
    pub fn from_input(input: String) -> Result<Self, &'static str> {
        Ok(Self {
            id: extract_game_id(input.clone())?,
            results: extract_game_results(input)?,
        })
    }

    pub fn is_possible(&self, limiting_result: &GameResult) -> bool {
        for result in self.results.clone() {
            if !result.is_possible(limiting_result) {
                return false;
            }
        }
        true
    }

    pub fn calculate_minimum_result(&self) -> GameResult {
        let mut minimum_result = GameResult::new(0, 0, 0);

        for result in self.results.clone() {
            minimum_result.red = minimum_result.red.max(result.red);
            minimum_result.blue = minimum_result.blue.max(result.blue);
            minimum_result.green = minimum_result.green.max(result.green);
        }

        minimum_result
    }
}

#[derive(Clone, Copy)]
struct GameResult {
    red: u32,
    green: u32,
    blue: u32,
}

impl GameResult {
    pub fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }

    pub fn from_input(input: String) -> Self {
        let red_regex = Regex::new(r"[0-9]+ red").unwrap();
        let blue_regex = Regex::new(r"[0-9]+ blue").unwrap();
        let green_regex = Regex::new(r"[0-9]+ green").unwrap();

        let red_match = red_regex.find(input.as_str());
        let blue_match = blue_regex.find(input.as_str());
        let green_match = green_regex.find(input.as_str());

        let red_substr = unwrap_color_substr(red_match, "red");
        let blue_substr = unwrap_color_substr(blue_match, "blue");
        let green_substr = unwrap_color_substr(green_match, "green");

        Self {
            red: extract_color_count(red_substr).unwrap_or(0),
            blue: extract_color_count(blue_substr).unwrap_or(0),
            green: extract_color_count(green_substr).unwrap_or(0),
        }
    }

    pub fn is_possible(&self, limiting_result: &GameResult) -> bool {
        self.red <= limiting_result.red
            && self.green <= limiting_result.green
            && self.blue <= limiting_result.blue
    }

    pub fn power(&self) -> u32 {
        self.red * self.blue * self.green
    }
}

pub fn day_two(input: String) -> u32 {
    let expected_result = GameResult::new(12, 13, 14);

    let games = convert_input_to_results(input);
    let total: u32 = games
        .iter()
        .filter(|game| game.is_possible(&expected_result))
        .map(|game| game.id)
        .sum();

    total
}

pub fn day_two_part_two(input: String) -> u32 {
    let games = convert_input_to_results(input);
    let total_power: u32 = games
        .into_iter()
        .map(|game| game.calculate_minimum_result())
        .map(|result| result.power())
        .sum();

    total_power
}

fn convert_input_to_results(input: String) -> Vec<Game> {
    let lines = input.lines();
    let games: Vec<Game> = lines
        .map(|f| Game::from_input(f.to_string()))
        .filter_map(|f| f.ok())
        .collect();

    games
}

fn extract_game_id(input: String) -> Result<u32, &'static str> {
    let game_regex = Regex::new(r"^Game [0-9]+:").unwrap();
    let game_substr = match game_regex.find(input.as_str()) {
        Some(x) => x,
        None => return Err("Malformed string: 'Game #:' not found."),
    };

    let game_id_regex = Regex::new(r"[0-9]+").unwrap();
    let game_id = match game_id_regex.find(game_substr.as_str()) {
        Some(x) => x.as_str(),
        None => return Err("Malformed string: Game ID is invalid"),
    };

    let game_id = match game_id.parse::<u32>() {
        Ok(x) => x,
        Err(_) => return Err("Could not convert game_id to a number"),
    };

    Ok(game_id)
}

fn extract_game_results(input: String) -> Result<Vec<GameResult>, &'static str> {
    let game_and_results: Vec<&str> = input.split(':').collect();

    let results = match game_and_results.get(1) {
        Some(x) => *x,
        None => return Err("Malformed input: no results"),
    };

    let individual_results: Vec<&str> = results.split(';').collect();
    let game_results: Vec<GameResult> = individual_results
        .iter()
        .map(|f| GameResult::from_input(f.to_string()))
        .collect();

    Ok(game_results)
}

fn unwrap_color_substr(possible_match: Option<Match<'_>>, color: &str) -> String {
    match possible_match {
        Some(color_match) => color_match.as_str().to_string(),
        None => format!("0 {}", color),
    }
}

fn extract_color_count(input: String) -> Result<u32, &'static str> {
    let re = Regex::new(r"[0-9]+").unwrap();
    let number = match re.find(input.as_str()) {
        Some(x) => x.as_str(),
        None => "0",
    };

    let number = match number.parse::<u32>() {
        Ok(number) => number,
        Err(_) => return Err("Could not parse the color count"),
    };
    Ok(number)
}
