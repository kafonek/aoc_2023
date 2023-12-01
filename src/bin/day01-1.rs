use aoc_2023::utils::{get_data_path, read_lines};
use regex::Regex;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let path = get_data_path();
    println!("Reading data from: {:?}", &path);
    let lines = read_lines(&path);

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
    println!("Sum: {}", sum);
    println!("Time: {:?}", now.elapsed());
}
