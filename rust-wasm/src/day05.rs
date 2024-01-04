use std::{collections::HashMap, ops::Range};

use aoc_2023::day05::Pipeline;

pub fn part1(input: String) -> String {
    let lines: Vec<String> = input.lines().map(|l| l.to_string()).collect();

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

pub fn part2(input: String) -> String {
    let lines: Vec<String> = input.lines().map(|l| l.to_string()).collect();

    // input looks like: seeds: 1972667147 405592018 1450194064 27782252
    // this time though we need to iterate over every two numbers where the first is a start value
    // and the second is a range to check over
    let parts = lines[0].split_once(':').unwrap();
    let mut seeds: Vec<usize> = parts
        .1
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let pipeline = Pipeline::from_lines(lines[2..].to_vec());

    let mut input_ranges: Vec<Range<usize>> = Vec::new();
    while !seeds.is_empty() {
        let start = seeds.remove(0);
        let range_length = seeds.remove(0);
        input_ranges.push(start..start + range_length);
    }
    println!("Input ranges: {:?}", input_ranges);
    let total_items: usize = input_ranges.iter().map(|r| r.end - r.start).sum();
    println!("Total number of items in flattened ranges: {}", total_items);

    // no rayon in wasm. Iterate through everything without parallelization
    let flattened = input_ranges.into_iter().flat_map(|r| r);
    let answer = flattened
        .into_iter()
        .map(|seed| pipeline.get(seed, false))
        .reduce(|a, b| a.min(b))
        .unwrap();
    answer.to_string()
}
