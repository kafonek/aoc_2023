/*
https://adventofcode.com/2023/day/1 part two
 - convert all spelled-out numbers to single-digit numbers
   - words may have overlapping letters, e.g. eightwo should be turned into 82
 - find the first and last digit in each line of the data file
 - combine those to make a two-digit integer (if there's only one digit, use it twice)
 - sum all the two-digit integers
*/
use aoc_2023::utils::run_and_time;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn solve(fp: &Path) -> String {
    let file = File::open(fp).expect("Failed to open data file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let tuples = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    let pattern: String = tuples
        .iter()
        .map(|(key, _)| *key)
        .chain(std::iter::once("\\d"))
        .collect::<Vec<&str>>()
        .join("|");
    let re = Regex::new(&pattern).unwrap();

    let maps: HashMap<_, _> = tuples.into_iter().collect();

    let mut extracted_digits: Vec<Vec<String>> = Vec::new();
    for line in &lines {
        // Biggest wrinkle in this problem: regex matches can overlap, e.g. eighthree matches 8 & 3
        // So hack is to not use default regex iteration but find a match then restart search from
        // match start index + 1
        // Not starting from end index -1 in case we match a single digit integer, that would end
        // up in an infinite loop.
        let mut matches = Vec::new();
        let mut start = 0;
        while let Some(matched) = re.find(&line[start..]) {
            let word_or_digit = matched.as_str();
            start += matched.start() + 1;

            let map_value = maps.get(word_or_digit).unwrap_or(&word_or_digit);
            matches.push(map_value.to_string());
        }
        extracted_digits.push(matches);
    }

    // Dev note: not sure how I feel about these nested function calls (closures?).
    // I think I like them?
    // Basically get the first and last items in each list in the list, and convert from spelled-out
    // number to single-digit number if that was what the regex matched on.
    let mut combines: Vec<String> = Vec::new();
    for item in &extracted_digits {
        let combine = {
            let first = {
                let i = item.first().unwrap();
                maps.get(i.as_str()).unwrap_or(&i.as_str()).to_string()
            };
            let second = {
                let i = item.last().unwrap();
                maps.get(i.as_str()).unwrap_or(&i.as_str()).to_string()
            };
            format!("{}{}", first, second)
        };
        combines.push(combine);
    }

    // Convert from strings to ints and sum it up
    let combines_n = combines
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let sum = combines_n.iter().sum::<i32>();
    sum.to_string()
}

fn main() {
    let path = Path::new("data/day01.txt");
    run_and_time(solve, path)
}
