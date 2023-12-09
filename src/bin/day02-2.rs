/*
https://adventofcode.com/2023/day/2 part two
 - Find max red, green, blue values among all bags in each game
 - Create "power" value for each game by multiplying max red, green, blue values
 - Sum all "power" values
*/
use aoc_2023::rust::day02::Game;
use aoc_2023::utils::run_and_time;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn solve(fp: &Path) -> String {
    let file = File::open(fp).expect("Failed to open data file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let mut games = Vec::new();
    for line in &lines {
        let game = Game::from_string(line).unwrap();
        games.push(game);
    }

    let mut answer = 0;
    for game in &games {
        answer += game.max_values().power();
    }

    answer.to_string()
}

fn main() {
    let fp = Path::new("data/day02.txt");
    run_and_time(solve, fp)
}
