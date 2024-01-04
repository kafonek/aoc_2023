use std::collections::HashMap;

use aoc_2023::day04::Card;

pub fn part1(input: String) -> String {
    let lines = input.lines();

    let mut cards = Vec::new();
    for line in lines {
        let card = Card::from_string(line);
        cards.push(card);
    }

    let mut answer: i32 = 0;
    for card in &cards {
        answer += card.score();
    }
    answer.to_string()
}

pub fn part2(input: String) -> String {
    let lines = input.lines();

    let mut cards = Vec::new();
    for line in lines {
        let card = Card::from_string(line);
        cards.push(card);
    }

    // basically {card.id: 1 for card in cards}
    let mut counts: HashMap<i32, i32> = cards.iter().map(|card| (card.id, 1)).collect();

    for card in &cards {
        let mult = *counts.get(&card.id).unwrap();
        for copy in card.copies() {
            if let Some(count) = counts.get_mut(&copy) {
                *count += mult;
            }
        }
    }
    let answer = counts.values().sum::<i32>();
    answer.to_string()
}
