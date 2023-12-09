use aoc_2023::rust::day08::Point;

use aoc_2023::utils::run_and_time;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::{collections::HashMap, fs::File};

fn solve(fp: &Path) -> String {
    let file = File::open(fp).expect("Failed to open data file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let mut pattern = lines[0].chars().cycle();

    let mut points: HashMap<String, Point> = HashMap::new();
    for line in &lines[2..] {
        let point = Point::from_string(&line);
        points.insert(point.name.clone(), point);
    }
    // println!("Points: {:?}", points);
    let mut current: Point = points.get("AAA").unwrap().clone();
    let mut steps: i32 = 0;

    loop {
        let direction = pattern.next().unwrap();
        if direction == 'R' {
            current = points.get(&current.right).unwrap().clone();
        } else {
            current = points.get(&current.left).unwrap().clone();
        }
        steps += 1;
        if &current.name == "ZZZ" {
            break;
        }
    }
    steps.to_string()
}

fn main() {
    let path = Path::new("./data/day08.txt");
    run_and_time(solve, path)
}
