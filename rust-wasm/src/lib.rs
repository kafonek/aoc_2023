mod day01;
mod day02;
mod day04;
mod day05;
mod day06;
mod day07;
use instant::Instant;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub fn solve(textarea: String, day_and_part: String) -> String {
    let start = Instant::now();
    console::log_1(&format!("Solving {}...", day_and_part).into());

    let answer = match day_and_part.as_str() {
        "day01_part1" => day01::part1(textarea),
        "day01_part2" => day01::part2(textarea),
        "day02_part1" => day02::part1(textarea),
        "day02_part2" => day02::part2(textarea),
        "day04_part1" => day04::part1(textarea),
        "day04_part2" => day04::part2(textarea),
        "day05_part1" => day05::part1(textarea),
        "day05_part2" => day05::part2(textarea),
        "day06_part1" => day06::part1(textarea),
        "day06_part2" => day06::part2(textarea),
        "day07_part1" => day07::part1(textarea),
        "day07_part2" => day07::part2(textarea),
        _ => "Not implemented".to_string(),
    };
    let elapsed = start.elapsed();
    format!("Answer: {}\nTime: {:?}", answer, elapsed)
}
