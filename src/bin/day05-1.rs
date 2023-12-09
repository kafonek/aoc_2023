use aoc_2023::rust::day05::Pipeline;
use aoc_2023::utils::run_and_time;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn solve(fp: &Path) -> String {
    let file = File::open(fp).expect("Failed to open data file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    // input looks like: seeds: 1972667147 405592018 1450194064 27782252
    // split on "seeds:" then split on whitespace and cast each to usize
    let parts = lines[0].split_once(':').unwrap();
    let seeds: Vec<usize> = parts
        .1
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let pipeline = Pipeline::from_lines(lines[2..].to_vec());

    let mut answer = usize::MAX;
    for seed in seeds {
        let result = pipeline.get(seed, false);
        if result < answer {
            answer = result;
        }
    }
    answer.to_string()
}

fn main() {
    let path = Path::new("./data/day05.txt");
    run_and_time(solve, path)
}
