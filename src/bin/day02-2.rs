use aoc_2023::utils::day02::Game;
use std::time::Instant;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let now = Instant::now();
    let path = Path::new("./data/day02.txt");
    println!("Reading data from: {:?}", &path);
    let file = File::open(&path).expect("Failed to open data file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let mut games = Vec::new();
    for line in &lines {
        let game = Game::from_string(line).unwrap();
        games.push(game);
    }

    let mut power_sum = 0;
    for game in &games {
        power_sum += game.max_values().power();
    }

    println!("Answer: {}", power_sum);
    println!("Time: {:?}", now.elapsed());
}
