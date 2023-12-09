use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Point {
    name: String,
    left: String,
    right: String,
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

        Point { name, left, right }
    }
}

#[derive(Debug)]
pub struct PointIndex {
    left: usize,
    right: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
pub struct PointMap {
    points: Vec<Point>,
    pub index_map: HashMap<String, usize>,
    pub point_indices: Vec<PointIndex>,
}

impl PointMap {
    pub fn new() -> Self {
        PointMap {
            points: Vec::new(),
            index_map: HashMap::new(),
            point_indices: Vec::new(),
        }
    }

    pub fn add_point(&mut self, point: Point) {
        let idx = self.points.len();
        self.index_map.insert(point.name.clone(), idx);
        self.points.push(point);
    }

    pub fn build_indices(&mut self) {
        for point in &self.points {
            let left_idx = *self.index_map.get(&point.left).unwrap_or(&usize::MAX);
            let right_idx = *self.index_map.get(&point.right).unwrap_or(&usize::MAX);
            self.point_indices.push(PointIndex {
                left: left_idx,
                right: right_idx,
            });
        }
    }

    pub fn get_index(&self, name: &str) -> usize {
        *self.index_map.get(name).unwrap_or(&usize::MAX)
    }

    pub fn navigate(&self, start_idx: usize, direction: Direction) -> usize {
        let point_index = &self.point_indices[start_idx];
        match direction {
            Direction::Left => point_index.left,
            Direction::Right => point_index.right,
        }
    }
}
