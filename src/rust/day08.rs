use std::borrow::Borrow;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Point {
    pub name: String,
    pub left: String,
    pub right: String,
    pub ends_with_z: bool,
}

impl Point {
    pub fn from_string(s: &str) -> Point {
        let parts: Vec<&str> = s.split('=').collect();
        let name = parts[0].trim().to_string();
        let values: Vec<&str> = parts[1]
            .trim()
            .trim_matches(|c| c == '(' || c == ')')
            .split(',')
            .collect();
        let left = values[0].trim().to_string();
        let right = values[1].trim().to_string();
        let ends_with_z = name.ends_with("Z");

        Point {
            name,
            left,
            right,
            ends_with_z,
        }
    }
}

#[derive(Debug)]
pub struct Traverse<'a> {
    pub current: &'a Point,
}

impl<'a> Traverse<'a> {
    pub fn new(start: &'a Point) -> Traverse<'a> {
        Traverse { current: start }
    }

    pub fn step(&mut self, direction: char, points: &'a HashMap<String, Point>) {
        if direction == 'R' {
            self.current = points.get(&self.current.right).unwrap();
        } else {
            self.current = points.get(&self.current.left).unwrap();
        }
    }
}
