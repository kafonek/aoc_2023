/*
https://adventofcode.com/2023/day/3 part one
 - find all contiguous numbers in the grid
 - identify all * in the grid touching exactly two numbers
 - multiply those numbers together and sum all those products
*/
use aoc_2023::utils::day03::Number;
use gridthings::{Cell, Grid, GridFromString};

use std::time::Instant;
use std::{collections::HashMap, path::Path};

fn main() {
    let now = Instant::now();
    let path = Path::new("./data/day03.txt");
    println!("Reading data from: {:?}", &path);

    let text = std::fs::read_to_string(&path).expect("Failed to read data file");
    let grid: Grid<char> = Grid::from_string(&text);

    // Find all contiguous numbers in each row
    let mut numbers = Vec::new();
    let mut current_collection = Vec::new();
    for row in grid.rows() {
        for cell in row {
            if cell.value.is_digit(10) {
                current_collection.push(cell.clone());
            } else if !current_collection.is_empty() {
                numbers.push(Number::new(current_collection.clone()));
                current_collection.clear();
            }
        }
        if !current_collection.is_empty() {
            numbers.push(Number::new(current_collection.clone()));
            current_collection.clear();
        }
    }

    // Find all * in the grid touching exactly two numbers
    let mut gears: HashMap<Cell<char>, Vec<Number>> = HashMap::new();
    for number in &numbers {
        for gear in number.gears(&grid) {
            gears.entry(gear).or_insert(Vec::new()).push(number.clone());
        }
    }

    let mut answer: i32 = 0;
    for (_gear, numbers) in gears {
        if numbers.len() == 2 {
            answer += numbers[0].value() * numbers[1].value();
        }
    }

    println!("Answer: {}", answer);
    println!("Time: {:?}", now.elapsed());
}
