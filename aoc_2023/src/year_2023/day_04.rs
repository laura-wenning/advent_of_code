pub fn day_04_part_1(input: String) -> u32 {
    let lotto_cards: Vec<LottoCard> = input
        .lines()
        .map(LottoCard::from)
        .filter_map(|card| card.ok())
        .collect();

    let winnings: u32 = lotto_cards
        .into_iter()
        .map(|f| f.get_wins())
        .filter(|f| *f > 0)
        .map(|f| 2_u32.pow(f - 1))
        .sum();
    winnings
}

struct LottoCard {
    winning_numbers: Vec<u32>,
    actual_numbers: Vec<u32>,
}

impl LottoCard {
    pub fn from(line: &str) -> Result<Self, &'static str> {
        let all_numbers: &str = match line.split(':').collect::<Vec<&str>>().get(1) {
            Some(x) => x,
            None => return Err("LottoCard input is incorrectly formatted"),
        };

        let binding = all_numbers.split('|').collect::<Vec<&str>>();
        let numbers: &[&str] = binding.as_slice();

        let winning_numbers: Vec<u32> = numbers[0]
            .split(' ')
            .map(|f| f.parse::<u32>())
            .filter_map(|f| f.ok())
            .collect();

        let actual_numbers: Vec<u32> = numbers[1]
            .split(' ')
            .map(|f| f.parse::<u32>())
            .filter_map(|f| f.ok())
            .collect();

        Ok(Self {
            winning_numbers,
            actual_numbers,
        })
    }

    pub fn get_wins(&self) -> u32 {
        let mut wins = 0;
        for actual_number in self.actual_numbers.clone() {
            for winning_number in self.winning_numbers.clone() {
                if actual_number == winning_number {
                    wins += 1;
                    break;
                }
            }
        }
        wins
    }
}
