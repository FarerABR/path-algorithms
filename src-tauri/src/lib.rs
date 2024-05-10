use std::time::Instant;

use serde::{Deserialize, Serialize};

pub fn ser_to_cell(arr: &Vec<Vec<String>>) -> Vec<Vec<CellType>> {
    let mut out: Vec<Vec<CellType>> = vec![vec![CellType::Blank; arr[0].len()]; arr.len()];

    for i in 0..arr.len() {
        for j in 0..arr[0].len() {
            out[i][j] = match arr[i][j].as_str() {
                "start" => CellType::Start,
                "destination" => CellType::Destination,
                "blank" => CellType::Blank,
                "block" => CellType::Block,
                &_ => CellType::Block,
            }
        }
    }
    out
}

pub fn ser_to_string(arr: &Vec<Vec<CellType>>) -> Vec<Vec<String>> {
    let mut out: Vec<Vec<String>> = vec![vec![String::new(); arr[0].len()]; arr.len()];
    for i in 0..arr.len() {
        for j in 0..arr[0].len() {
            out[i][j] = match arr[i][j] {
                CellType::Start => "start".to_string(),
                CellType::Destination => "destination".to_string(),
                CellType::Blank => "blank".to_string(),
                CellType::Block => "block".to_string(),
                CellType::Visited(i) => format!("path-{}", i),
            }
        }
    }
    out
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CellType {
    Blank,
    Start,
    Destination,
    Block,
    Visited(u32), // Includes path number
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
enum Error {
    DeadEnd,
}
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub struct Grid {
    pub cells: Vec<Vec<CellType>>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(cells: Vec<Vec<CellType>>) -> Self {
        let height = cells.len();
        let width = cells[0].len();
        Self {
            cells,
            width,
            height,
        }
    }

    fn is_within_bounds(&self, point: Point) -> bool {
        point.x < self.width && point.y < self.height
    }

    #[doc = "dfs function for finding path"]
    #[doc = "\nThe output is (path, time)"]
    /// asdas
    fn dfs(&mut self, start_point: Point) -> Option<(Vec<Point>, f32)> {
        let time = Instant::now();
        let mut stack = Vec::<Point>::new();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; self.width]; self.height];
        let mut path = Vec::<Point>::new();

        stack.push(start_point);

        while let Some(current) = stack.pop() {
            let x = current.x;
            let y = current.y;

            visited[y][x] = true;
            path.push(Point { x: y, y: x });

            if self.cells[y][x] == CellType::Destination {
                // the path is currenly transvesed
                let mut path_num = 1;
                for node in &path {
                    self.cells[node.x][node.y] = CellType::Visited(path_num);
                    path_num += 1;
                }
                let duration = time.elapsed();
                return Some((path, duration.as_secs_f32()));
            }

            let directions = [
                (-1, 0), // Up
                (0, 1),  // Right
                (1, 0),  // Down
                (0, -1), // Left
            ];

            for (dx, dy) in &directions {
                let new_point = Point {
                    x: (x as isize + dx) as usize,
                    y: (y as isize + dy) as usize,
                };
                if self.is_within_bounds(new_point)
                    && !visited[new_point.y][new_point.x]
                    && self.cells[new_point.y][new_point.x] != CellType::Block
                {
                    stack.push(new_point)
                }
            }
        }
        None // No path found
    }
    pub fn solve(&mut self, start: Point) {
        self.dfs(start);
    }
}
