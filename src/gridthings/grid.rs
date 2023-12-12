use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cell<T>
where
    T: Clone + PartialEq + Eq + Hash,
{
    pub y: usize,
    pub x: usize,
    pub value: T,
}

impl<T> Cell<T>
where
    T: Clone + PartialEq + Eq + Hash,
{
    pub fn new(y: usize, x: usize, value: T) -> Self {
        Cell { y, x, value }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T>
where
    T: Clone + PartialEq + Eq + Hash,
{
    pub data: Vec<Vec<T>>,
}

impl<T> Grid<T>
where
    T: Clone + PartialEq + Eq + Hash,
{
    pub fn new(data: Vec<Vec<T>>) -> Self {
        Grid { data }
    }

    pub fn get_cell(&self, y: usize, x: usize) -> Option<Cell<T>> {
        if y < self.data.len() && x < self.data[y].len() {
            Some(Cell::new(y, x, self.data[y][x].clone()))
        } else {
            None
        }
    }

    pub fn get_row(&self, y: usize) -> Vec<Cell<T>> {
        let mut row: Vec<Cell<T>> = Vec::new();
        for x in 0..self.data[y].len() {
            row.push(self.get_cell(y, x).unwrap());
        }
        row
    }

    pub fn get_col(&self, x: usize) -> Vec<Cell<T>> {
        let mut col: Vec<Cell<T>> = Vec::new();
        for y in 0..self.data.len() {
            col.push(self.get_cell(y, x).unwrap());
        }
        col
    }

    pub fn insert_row(&mut self, y: usize, row: Vec<T>) {
        self.data.insert(y, row);
    }

    pub fn insert_col(&mut self, x: usize, col: Vec<T>) {
        for y in 0..self.data.len() {
            self.data[y].insert(x, col[y].clone());
        }
    }

    pub fn peek_horizontal(&self, y: usize, x: usize, offset: usize) -> Vec<Cell<T>> {
        let mut results: Vec<Cell<T>> = Vec::new();
        if let Some(look_left) = x.checked_sub(offset) {
            if let Some(cell) = self.get_cell(y, look_left) {
                results.push(cell);
            }
        }
        if let Some(look_right) = x.checked_add(offset) {
            if let Some(cell) = self.get_cell(y, look_right) {
                results.push(cell);
            }
        }
        results
    }

    pub fn peek_vertical(&self, y: usize, x: usize, offset: usize) -> Vec<Cell<T>> {
        let mut results: Vec<Cell<T>> = Vec::new();
        if let Some(look_up) = y.checked_sub(offset) {
            if let Some(cell) = self.get_cell(look_up, x) {
                results.push(cell);
            }
        }
        if let Some(look_down) = y.checked_add(offset) {
            if let Some(cell) = self.get_cell(look_down, x) {
                results.push(cell);
            }
        }
        results
    }

    pub fn peek_linear(&self, y: usize, x: usize, offset: usize) -> Vec<Cell<T>> {
        let mut results: Vec<Cell<T>> = Vec::new();
        results.append(&mut self.peek_horizontal(y, x, offset));
        results.append(&mut self.peek_vertical(y, x, offset));
        results
    }

    pub fn peek_diagonal(&self, y: usize, x: usize, offset: usize) -> Vec<Cell<T>> {
        let mut results: Vec<Cell<T>> = Vec::new();
        if let Some(look_up) = y.checked_sub(offset) {
            if let Some(look_left) = x.checked_sub(offset) {
                if let Some(cell) = self.get_cell(look_up, look_left) {
                    results.push(cell);
                }
            }
            if let Some(look_right) = x.checked_add(offset) {
                if let Some(cell) = self.get_cell(look_up, look_right) {
                    results.push(cell);
                }
            }
        }
        if let Some(look_down) = y.checked_add(offset) {
            if let Some(look_left) = x.checked_sub(offset) {
                if let Some(cell) = self.get_cell(look_down, look_left) {
                    results.push(cell);
                }
            }
            if let Some(look_right) = x.checked_add(offset) {
                if let Some(cell) = self.get_cell(look_down, look_right) {
                    results.push(cell);
                }
            }
        }
        results
    }

    pub fn peek_all(&self, y: usize, x: usize, offset: usize) -> Vec<Cell<T>> {
        let mut results: Vec<Cell<T>> = Vec::new();
        results.append(&mut self.peek_linear(y, x, offset));
        results.append(&mut self.peek_diagonal(y, x, offset));
        results
    }
}

pub trait GridFromString<T> {
    fn from_chars(s: &str) -> Self;
    fn from_delimiter(s: &str, delimiter: &str) -> Self;
}

impl GridFromString<char> for Grid<char> {
    fn from_chars(s: &str) -> Self {
        let mut data: Vec<Vec<char>> = Vec::new();
        for line in s.lines() {
            let mut row: Vec<char> = Vec::new();
            for c in line.chars() {
                row.push(c);
            }
            data.push(row);
        }
        Grid::new(data)
    }

    fn from_delimiter(s: &str, delimiter: &str) -> Self {
        let mut data: Vec<Vec<char>> = Vec::new();
        for line in s.lines() {
            let mut row: Vec<char> = Vec::new();
            for c in line.split(delimiter) {
                row.push(c.chars().next().unwrap());
            }
            data.push(row);
        }
        Grid::new(data)
    }
}

impl GridFromString<i32> for Grid<i32> {
    fn from_chars(s: &str) -> Self {
        let mut data: Vec<Vec<i32>> = Vec::new();
        for line in s.lines() {
            let mut row: Vec<i32> = Vec::new();
            for c in line.chars() {
                row.push(c.to_digit(10).unwrap() as i32);
            }
            data.push(row);
        }
        Grid::new(data)
    }

    fn from_delimiter(s: &str, delimiter: &str) -> Self {
        let mut data: Vec<Vec<i32>> = Vec::new();
        for line in s.lines() {
            let mut row: Vec<i32> = Vec::new();
            for c in line.split(delimiter) {
                row.push(c.parse::<i32>().unwrap());
            }
            data.push(row);
        }
        Grid::new(data)
    }
}
