use aoc_2023::rust::day05::Pipeline;
use rayon::prelude::*;

use aoc_2023::utils::run_and_time;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;
use std::path::Path;

fn solve(fp: &Path) -> String {
    let file = File::open(&fp).expect("Failed to open data file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    // input looks like: seeds: 1972667147 405592018 1450194064 27782252
    // this time though we need to iterate over every two numbers where the first is a start value
    // and the second is a range to check over
    let parts = lines[0].split_once(":").unwrap();
    let mut seeds: Vec<usize> = parts
        .1
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let pipeline = Pipeline::from_lines(lines[2..].to_vec());

    let mut input_ranges: Vec<Range<usize>> = Vec::new();
    while seeds.len() > 0 {
        let start = seeds.remove(0);
        let range_length = seeds.remove(0);
        input_ranges.push(start..start + range_length);
    }
    println!("Input ranges: {:?}", input_ranges);
    let total_items: usize = input_ranges.iter().map(|r| r.end - r.start).sum();
    println!("Total number of items in flattened ranges: {}", total_items);

    let flattened = input_ranges.into_par_iter().flat_map_iter(|r| r);

    let answer = flattened
        .into_par_iter()
        .map(|seed| pipeline.get(seed, false))
        .reduce(|| usize::MAX, |a, b| a.min(b));
    answer.to_string()
}

fn main() {
    let path = Path::new("./data/day05.txt");
    run_and_time(solve, path)
}
