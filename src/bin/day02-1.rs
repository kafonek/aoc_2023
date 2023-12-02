use aoc_2023::helpers::day02::{Bag, Game};
use aoc_2023::utils::{get_data_path, read_lines};
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let path = get_data_path();
    println!("Reading data from: {:?}", &path);
    let lines = read_lines(&path);
    let benchmark = Bag::new(14, 12, 13);

    let mut games = Vec::new();
    for line in &lines {
        let game = Game::from_string(&line).unwrap();
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
