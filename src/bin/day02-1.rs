use aoc_2023::utils::day02::{Bag, Game};
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
    // "max bag" from the problem statement
    let benchmark = Bag::new(14, 12, 13);

    let mut games = Vec::new();
    for line in &lines {
        let game = Game::from_string(line).unwrap();
        games.push(game);
    }

    let mut id_sum = 0;
    for game in &games {
        if game.check(&benchmark) {
            id_sum += &game.id
        }
    }

    println!("Answer: {}", id_sum);
    println!("Time: {:?}", now.elapsed());
}
