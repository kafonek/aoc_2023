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

    let times: Vec<usize> = lines[0]
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .into_iter()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let distances: Vec<usize> = lines[1]
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .into_iter()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut races: Vec<Race> = Vec::new();
    // zip times and distances together
    for (time, distance) in times.iter().zip(distances.iter()) {
        let mut r = Race::new(*time, *distance);
        r.run();
        races.push(r);
    }
    let answer = races.iter().fold(1, |acc, r| acc * r.answer);
    println!("Answer: {}", answer);
    println!("Time: {:?}", now.elapsed());
}
