/*
https://adventofcode.com/2023/day/3 part one
 - find all contiguous numbers in the grid
 - identify which ones are touching a symbol (non-digit, non-. value)
 - sum all those numbers
*/
// use aoc_2023::rust::day03::Number;
// use aoc_2023::utils::run_and_time;
// use gridthings::{Grid, GridFromString};

// use std::path::Path;

// fn solve(fp: &Path) -> String {
//     let text = std::fs::read_to_string(fp).expect("Failed to read data file");
//     let grid: Grid<char> = Grid::from_string(&text);

//     // Find all contiguous numbers in each row
//     let mut numbers = Vec::new();
//     let mut current_collection = Vec::new();
//     for row in grid.rows() {
//         for cell in row {
//             if cell.value.is_ascii_digit() {
//                 current_collection.push(cell.clone());
//             } else if !current_collection.is_empty() {
//                 numbers.push(Number::new(current_collection.clone()));
//                 current_collection.clear();
//             }
//         }
//         if !current_collection.is_empty() {
//             numbers.push(Number::new(current_collection.clone()));
//             current_collection.clear();
//         }
//     }

//     let mut answer: i32 = 0;
//     for number in &numbers {
//         if number.symbol_adjacent(&grid) {
//             answer += number.value();
//         }
//     }
//     answer.to_string()
// }

// fn main() {
//     let path = Path::new("data/day03.txt");
//     run_and_time(solve, path)
// }
