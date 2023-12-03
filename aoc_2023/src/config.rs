use std::env;

use crate::challenge::ChallengeIdentifier;

pub struct Config {
    pub program_identifier: ChallengeIdentifier,
    pub as_test: bool,
}

impl Config {
    pub fn load() -> Self {
        let args: Vec<String> = env::args().collect();

        let mut state = Self {
            program_identifier: ChallengeIdentifier::new(2023, 1, 1),
            as_test: false,
        };

        let err_message = "Invalid argument. Usage: cargo run -- [day] [part] [use_test_data]";

        let day = args.get(1);
        if let Some(day) = day {
            state.program_identifier.day = day.parse::<u32>().expect(err_message);
        }

        let part = args.get(2);
        if let Some(part) = part {
            state.program_identifier.part = part.parse::<u32>().expect(err_message);
        }

        let use_test_data = args.get(3);
        if let Some(use_test_data) = use_test_data {
            state.as_test = use_test_data == "true";
        }

        state
    }
}
