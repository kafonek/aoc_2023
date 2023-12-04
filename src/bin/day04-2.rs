use aoc_2023::utils::day04::Card;
use std::collections::HashMap;
use std::time::Instant;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let now = Instant::now();
    let path = Path::new("./data/day04.txt");
    println!("Reading data from: {:?}", &path);
    let file = File::open(&path).expect("Failed to open data file");
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
    println!("Answer: {}", answer);
    println!("Time: {:?}", now.elapsed());
}
