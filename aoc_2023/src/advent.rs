use crate::{challenge::Challenge, config::Config, year_2023};

pub struct AdventOfCode {
    config: Config,
    challenges: Vec<Challenge>,
}

impl AdventOfCode {
    pub fn new() -> Self {
        let mut advent = Self {
            config: Config::load(),
            challenges: vec![],
        };
        advent.register_programs();
        advent
    }

    fn register_programs(&mut self) {
        year_2023::register(self);
    }

    pub fn register(&mut self, year: u32, day: u32, part: u32, function: fn(String) -> u32) {
        let challenge = Challenge::new(year, day, part, function);
        self.challenges.push(challenge);
    }

    pub fn run(&self) -> Result<(), String> {
        let challenge = self.get_challenge();
        let result = match challenge {
            Some(challenge) => challenge.run(self.config.as_test)?,
            None => return Err("No challenge could be found for the given input".to_string()),
        };
        println!("Result: {}", result);
        Ok(())
    }

    fn get_challenge(&self) -> Option<Challenge> {
        self.challenges
            .clone()
            .into_iter()
            .find(|challenge| challenge.identifier == self.config.program_identifier)
    }
}
