use std::collections::HashMap;

use aoc_2023::utils::{get_data_path, read_lines};
use regex::Regex;

fn main() {
    let path = get_data_path();
    println!("Reading data from: {:?}", &path);
    let lines = read_lines(&path);

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

    let combines_n = combines
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let sum = combines_n.iter().sum::<i32>();
    println!("Sum: {}", sum);
}
