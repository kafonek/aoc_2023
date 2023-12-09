use aoc_2023::rust::day08::{Point, Traverse};

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

    let mut traverses: Vec<Traverse> = Vec::new();
    for point in points.values() {
        if point.name.ends_with("A") {
            traverses.push(Traverse::new(&point));
        }
    }
    println!("Traverses: {:?}", traverses);

    let mut steps: i32 = 0;
    loop {
        let direction = pattern.next().unwrap();
        let mut complete = true;
        for traverse in &mut traverses {
            traverse.step(direction, &points);
            if !traverse.current.ends_with_z {
                complete = false;
            }
        }
        steps += 1;
        if steps % 10000 == 0 {
            println!("Steps: {}", steps);
        }
        if complete {
            break;
        }
    }
    steps.to_string()
}

fn main() {
    let path = Path::new("./data/day08.txt");
    run_and_time(solve, path)
}
