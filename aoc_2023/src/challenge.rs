use std::fs;

#[derive(Clone, PartialEq, Eq)]
pub struct ChallengeIdentifier {
    pub year: u32,
    pub day: u32,
    pub part: u32,
}

impl ChallengeIdentifier {
    pub fn new(year: u32, day: u32, part: u32) -> Self {
        ChallengeIdentifier { year, day, part }
    }
}

#[derive(Clone)]
pub struct Challenge {
    pub identifier: ChallengeIdentifier,
    function: fn(String) -> u32,
}

impl Challenge {
    pub fn new(year: u32, day: u32, part: u32, function: fn(String) -> u32) -> Self {
        Self {
            identifier: ChallengeIdentifier::new(year, day, part),
            function,
        }
    }

    pub fn run(&self, as_test: bool) -> Result<u32, String> {
        let input = self.load_input(as_test)?;
        Ok((self.function)(input))
    }

    fn load_input(&self, as_test: bool) -> Result<String, String> {
        let path = self.build_path(as_test);
        let input = match fs::read_to_string(path.clone()) {
            Err(_) => return Err(format!("Unable to read '{}'.", path)),
            Ok(x) => x,
        };
        Ok(input)
    }

    fn build_path(&self, as_test: bool) -> String {
        let year = self.identifier.year.to_string();
        let mut day = self.identifier.day.to_string();
        if day.len() == 1 {
            day = format!("0{}", day);
        }

        let test_postfix = if as_test { "_test" } else { "" };
        format!("./inputs/year_{}/day_{}{}.txt", year, day, test_postfix)
    }
}
