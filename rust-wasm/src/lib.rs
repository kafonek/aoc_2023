mod day01;
mod day02;
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
        _ => "Not implemented".to_string(),
    };
    let elapsed = start.elapsed();
    format!("Answer: {}\nTime: {:?}", answer, elapsed)
}
