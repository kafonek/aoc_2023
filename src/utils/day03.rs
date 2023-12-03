use gridthings::{Cell, Grid};

#[derive(Debug, Clone)]
pub struct Number {
    cells: Vec<Cell<char>>,
}

impl Number {
    pub fn new(cells: Vec<Cell<char>>) -> Self {
        Number { cells }
    }

    // Combine the char values of cells into a String then cast to i32
    pub fn value(&self) -> i32 {
        let mut s = String::new();
        for cell in &self.cells {
            s.push(cell.value);
        }
        s.parse::<i32>().unwrap()
    }

    // Find all adjacent cells to all the cells in this Number, not including cells that
    // are part of the Number itself, removing duplicates
    fn peek_all(&self, grid: &Grid<char>) -> Vec<Cell<char>> {
        let mut matches = Vec::new();
        for cell in &self.cells {
            let results = grid.peek_all(cell.y, cell.x, 1);
            for result in results {
                if !self.cells.contains(&result) && !matches.contains(&result) {
                    matches.push(result);
                }
            }
        }
        matches
    }

    // True if any of the cells are adjacent to a cell with a symbol (non-digit, non-.)
    pub fn symbol_adjacent(&self, grid: &Grid<char>) -> bool {
        for cell in self.peek_all(grid) {
            if !cell.value.is_digit(10) && cell.value != '.' {
                return true;
            }
        }
        false
    }

    // Return any adjacent cells that have value *
    pub fn gears(&self, grid: &Grid<char>) -> Vec<Cell<char>> {
        let mut gears = Vec::new();
        for cell in self.peek_all(grid) {
            if cell.value == '*' {
                gears.push(cell);
            }
        }
        gears
    }
}
