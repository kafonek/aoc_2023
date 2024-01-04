use aoc_2023::day01::Calibration;
use aoc_2023::utils::run_and_time;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn solve(fp: &Path) -> String {
    let file = File::open(fp).expect("Failed to open data file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let mut answer = 0;
    for line in lines {
        let calibration = Calibration::from_line(&line);
        answer += calibration.value();
    }
    answer.to_string()
}

fn main() {
    let path = Path::new("../data/day01.txt");
    run_and_time(solve, path);
}
