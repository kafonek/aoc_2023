/*
https://adventofcode.com/2023/day/2 part one
 - "max bag" is 12 red cubes, 13 green cubes, 14 blue cubes
 - for each game id in the dataset, see if all bags in the game can fit into the max bag
 - sum up game ids that fit the max bag criteria
*/
use aoc_2023::rust::day02::{Bag, Game};
use aoc_2023::utils::run_and_time;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn solve(fp: &Path) -> String {
    let file = File::open(fp).expect("Failed to open data file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let max_bag = Bag::new(12, 13, 14);

    let mut games = Vec::new();
    for line in &lines {
        let game = Game::from_string(line).unwrap();
        games.push(game);
    }

    let mut answer = 0;
    for game in &games {
        if game.check(&max_bag) {
            answer += &game.id
        }
    }

    answer.to_string()
}

fn main() {
    let fp = Path::new("data/day02.txt");
    run_and_time(solve, fp)
}
