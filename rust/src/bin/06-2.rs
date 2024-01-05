use aoc_2023::day06::Race;
use aoc_2023::utils::run_and_time;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn solve(fp: &Path) -> String {
    let file = File::open(fp).expect("Failed to open data file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    // Python equivalent: int("".join(lines[0].split(":")[1].split()))
    let time = lines[0]
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse::<usize>()
        .unwrap();

    let distance = lines[1]
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse::<usize>()
        .unwrap();

    let race = Race::new(time, distance);
    let answer = race.run();
    answer.to_string()
}

fn main() {
    let path = Path::new("../data/day06.txt");
    run_and_time(solve, path)
}
