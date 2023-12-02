use aoc_2023::helpers::day02::Game;
use aoc_2023::utils::{get_data_path, read_lines};
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let path = get_data_path();
    println!("Reading data from: {:?}", &path);
    let lines = read_lines(&path);

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
