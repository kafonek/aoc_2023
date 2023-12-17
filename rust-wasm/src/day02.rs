use aoc_2023::day02::{Bag, Game};

pub fn part1(input: String) -> String {
    let lines = input.lines();

    let max_bag = Bag::new(12, 13, 14);

    let mut games = Vec::new();
    for line in lines {
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

pub fn part2(input: String) -> String {
    let lines = input.lines();

    let mut games = Vec::new();
    for line in lines {
        let game = Game::from_string(line).unwrap();
        games.push(game);
    }

    let mut answer = 0;
    for game in &games {
        answer += game.max_values().power();
    }

    answer.to_string()
}
