use aoc_2023::rust::day04::Card;
use aoc_2023::utils::run_and_time;
use std::collections::HashMap;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn solve(fp: &Path) -> String {
    let file = File::open(&fp).expect("Failed to open data file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let mut cards = Vec::new();
    for line in &lines {
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
    let answer = counts.values().into_iter().sum::<i32>();
    answer.to_string()
}

fn main() {
    let path = Path::new("./data/day04.txt");
    run_and_time(solve, path)
}
