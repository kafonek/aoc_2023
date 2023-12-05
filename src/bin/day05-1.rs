use aoc_2023::utils::day05::Pipeline;

use std::time::Instant;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let now = Instant::now();
    let path = Path::new("./data/day05.txt");
    println!("Reading data from: {:?}", &path);
    let file = File::open(&path).expect("Failed to open data file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    // input looks like: seeds: 1972667147 405592018 1450194064 27782252
    // split on "seeds:" then split on whitespace and cast each to usize
    let parts = lines[0].split_once(":").unwrap();
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
            println!("{} -> {}", seed, result);
            answer = result;
        }
    }

    println!("Answer: {}", answer);
    println!("Time: {:?}", now.elapsed());
}
