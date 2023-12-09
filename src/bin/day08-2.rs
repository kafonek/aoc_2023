use aoc_2023::rust::day08::{Direction, Point, PointMap};
use aoc_2023::utils::run_and_time;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn solve(fp: &Path) -> String {
    let file = File::open(fp).expect("Failed to open data file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let directions: Vec<Direction> = lines[0]
        .chars()
        .filter_map(|c| match c {
            'R' => Some(Direction::Right),
            'L' => Some(Direction::Left),
            _ => None,
        })
        .collect();
    let mut pattern = directions.iter().cycle();

    let mut point_map = PointMap::new();
    for line in lines[2..].iter() {
        let point = Point::from_string(&line);
        point_map.add_point(point);
    }
    point_map.build_indices();

    let mut current_indices: Vec<usize> = Vec::new();
    let mut stop_indices: Vec<usize> = Vec::new();
    for (key, point) in point_map.index_map.iter() {
        if key.ends_with("A") {
            current_indices.push(*point);
        } else if key.ends_with("Z") {
            stop_indices.push(*point);
        }
    }
    println!("Current indices: {:?}", current_indices);
    println!("Stop indices: {:?}", stop_indices);

    let mut steps: usize = 0;
    loop {
        let direction = pattern.next().unwrap();
        let mut complete: bool = true;
        for idx in current_indices.iter_mut() {
            *idx = point_map.navigate(*idx, *direction);
            if !stop_indices.contains(idx) {
                complete = false;
            }
        }
        steps += 1;
        if steps % 1000000 == 0 {
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
