use aoc_2023::day07::Hand;
use aoc_2023::utils::run_and_time;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn solve(fp: &Path) -> String {
    let file = File::open(fp).expect("Failed to open data file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|l| {
            let line = l.unwrap();
            line.replace("J", "X")
        })
        .collect();

    let mut hands: Vec<Hand> = Vec::new();
    for line in lines {
        if let Some(hand) = Hand::from_string(line) {
            hands.push(hand);
        }
    }
    hands.sort();

    let mut answer = 0;
    for (idx, hand) in hands.iter().enumerate() {
        let power = idx + 1;
        answer += hand.bid * power;
    }
    answer.to_string()
}

fn main() {
    let path = Path::new("../data/day07.txt");
    run_and_time(solve, path)
}
