use aoc_2023::day01::Calibration;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn solve_01_1(textarea: String) -> String {
    let lines = textarea.lines();

    let mut answer = 0;
    for line in lines {
        let calibration = Calibration::from_line(&line);
        answer += calibration.value();
    }
    answer.to_string()
}

#[wasm_bindgen]
pub fn solve_01_2(textarea: String) -> String {
    let lines = textarea.lines();

    let mut answer = 0;
    for line in lines {
        let calibration = Calibration::from_line(&line);
        answer += calibration.value2();
    }
    answer.to_string()
}
