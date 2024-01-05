use aoc_2023::day06::Race;
use aoc_2023::utils::run_and_time;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn solve(fp: &Path) -> String {
    let file = File::open(fp).expect("Failed to open data file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    // Python equivalent: times = lines[0].split(":")[1].split()
    let times: Vec<usize> = lines[0]
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|t| t.parse().unwrap())
        .collect();
    let distances: Vec<usize> = lines[1]
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut results: Vec<usize> = Vec::new();
    // zip times and distances together
    for (time, distance) in times.iter().zip(distances.iter()) {
        let r = Race::new(*time, *distance);
        let result = r.run();
        results.push(result);
    }
    let answer = results.iter().product::<usize>();
    answer.to_string()
}

fn main() {
    let path = Path::new("../data/day06.txt");
    run_and_time(solve, path)
}
