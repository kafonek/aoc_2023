/*
https://adventofcode.com/2023/day/1 part one
 - find the first and last digit in each line of the data file
 - combine those to make a two-digit integer (if there's only one digit, use it twice)
 - sum all the two-digit integers
*/
use aoc_2023::utils::run_and_time;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn solve(fp: &Path) -> String {
    let file = File::open(&fp).expect("Failed to open data file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let re = Regex::new(r"\d").unwrap(); // match any single digit

    let mut extracted_digits: Vec<Vec<String>> = Vec::new();
    for line in &lines {
        let matches: Vec<String> = re.find_iter(line).map(|c| c.as_str().to_owned()).collect();
        extracted_digits.push(matches);
    }

    let mut combines: Vec<String> = Vec::new();
    for item in &extracted_digits {
        let combine = {
            let first = item.first().unwrap();
            let second = item.last().unwrap();
            format!("{}{}", first, second)
        };
        combines.push(combine);
    }

    // convert list of strings to list of i32
    let combines_n = combines
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let sum = combines_n.iter().sum::<i32>();
    sum.to_string()
}

fn main() {
    let fp = Path::new("data/day01.txt");
    run_and_time(solve, fp)
}
