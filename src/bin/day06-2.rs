use aoc_2023::rust::day06::Race;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let path = Path::new("./data/day06.txt");
    println!("Reading data from: {:?}", &path);
    let file = File::open(&path).expect("Failed to open data file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let time = lines[0]
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse::<usize>()
        .unwrap();

    let distance = lines[1]
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse::<usize>()
        .unwrap();

    let mut race = Race::new(time, distance);
    race.run();
    println!("Answer: {}", race.answer);
    println!("Time: {:?}", now.elapsed());
}
