use aoc_2023::rust::day04::Card;
use aoc_2023::utils::run_and_time;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn solve(fp: &Path) -> String {
    let file = File::open(&fp).expect("Failed to open data file");
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
    answer.to_string()
}
fn main() {
    let path = Path::new("./data/day04.txt");
    run_and_time(solve, path)
}
