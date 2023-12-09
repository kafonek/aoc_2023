#[derive(Debug)]
pub struct Card {
    pub id: i32,
    winning_numbers: Vec<i32>,
    guess_numbers: Vec<i32>,
}

impl Card {
    pub fn from_string(s: &str) -> Card {
        let parts: Vec<&str> = s.splitn(2, ':').collect();
        let card = parts[0];
        let numbers = parts[1];

        let id: i32 = card
            .split_whitespace()
            .nth(1)
            .expect("Error parsing Card ID")
            .parse()
            .expect("Error converting Card ID to i32");

        let number_parts: Vec<&str> = numbers.splitn(2, '|').collect();
        let winning_numbers: Vec<i32> = number_parts[0]
            .split_whitespace()
            .map(|n| n.parse().expect("Error converting winning number to i32"))
            .collect();
        let guess_numbers: Vec<i32> = number_parts[1]
            .split_whitespace()
            .map(|n| n.parse().expect("Error converting guess number to i32"))
            .collect();
        Card {
            id,
            winning_numbers,
            guess_numbers,
        }
    }

    pub fn score(&self) -> i32 {
        let mut result: i32 = 0;
        for n in &self.guess_numbers {
            if self.winning_numbers.contains(&n) {
                if result == 0 {
                    result = 1;
                } else {
                    result = result * 2
                }
            }
        }
        result
    }

    pub fn copies(&self) -> Vec<i32> {
        let copy_range = self
            .guess_numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count();
        let start = self.id + 1;
        let stop = start + copy_range as i32;
        (start..stop).collect()
    }
}
