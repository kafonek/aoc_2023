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

    let mut current_idx = point_map.get_index("AAA");
    let end_idx = point_map.get_index("ZZZ");
    let mut steps: i32 = 0;
    loop {
        let direction = pattern.next().unwrap();
        current_idx = point_map.navigate(current_idx, *direction);
        steps += 1;
        if current_idx == end_idx {
            break;
        }
    }

    steps.to_string()
}

fn main() {
    let path = Path::new("./data/day08.txt");
    run_and_time(solve, path)
}
