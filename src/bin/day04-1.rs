use aoc_2023::utils::day04::Card;
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

    let mut answer: i32 = 0;
    for card in &cards {
        answer += card.score();
    }
    println!("Answer: {}", answer);
    println!("Time: {:?}", now.elapsed());
}
