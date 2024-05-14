use std::{
    cell,
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, VecDeque},
    hash::Hash,
    time::Instant,
};

use rand::{rngs::StdRng, Rng};
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

#[allow(dead_code)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellType {
    Blank,
    Start,
    Destination,
    Block,
    Visited(u32), // Includes path number
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Eq, Hash, Ord, PartialOrd)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn as_tuple(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

#[derive(Debug)]
struct Cell {
    f: usize,
    h: usize,
    g: usize,
    parent: Option<Point>,
}
impl Cell {
    fn new() -> Self {
        Self {
            f: usize::MAX,
            h: usize::MAX,
            g: usize::MAX,
            parent: None,
        }
    }
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

    pub fn construct_grid(width: usize, height: usize) -> Self {
        let mut grid = vec![vec![CellType::Blank; height]; width];
        for i in 0..width {
            grid[i][0] = CellType::Block;
            grid[i][height - 1] = CellType::Block;
        }

        for j in 0..height {
            grid[0][j] = CellType::Block;
            grid[width - 1][j] = CellType::Block;
        }

        let mut rng = rand::thread_rng();

        // start point
        loop {
            let start = (
                rng.gen_range(0..(width - 1)),
                rng.gen_range(0..(height - 1)),
            );
            if !(grid[start.0][start.1] != CellType::Blank) {
                grid[start.0][start.1] = CellType::Start;
                break;
            }
        }

        // destination point
        loop {
            let destination = (
                rng.gen_range(0..(width - 1)),
                rng.gen_range(0..(height - 1)),
            );
            if !(grid[destination.0][destination.1] != CellType::Blank) {
                grid[destination.0][destination.1] = CellType::Destination;
                break;
            }
        }

        let block_num = rng.gen_range(1..((width - 2) * (height - 2) / 2));
        let mut placed = 0;
        println!("blocks: {}", block_num);
        while placed < block_num {
            let (x, y) = (
                rng.gen_range(0..(width - 1)),
                rng.gen_range(0..(height - 1)),
            );
            println!("placed: {}", placed);
            if grid[x][y] == CellType::Blank {
                grid[x][y] = CellType::Block;
                placed += 1;
            }
        }

        println!("{:?}", grid);
        Grid {
            cells: grid,
            width: width - 1,
            height: height - 1,
        }
    }

    fn is_within_bounds(&self, point: Point) -> bool {
        point.x < self.width && point.y < self.height
    }

    #[doc = "dfs function for finding path"]
    #[doc = "\nThe output is (path, time)"]
    pub fn dfs(&mut self, start_point: Point) -> Option<(Vec<Point>, f32)> {
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

            // removing the start_point from the path
            // doing it now so it could be done in O(1) exept O(N)
            if x == start_point.x && y == start_point.y {
                path.pop();
            }

            if self.cells[y][x] == CellType::Destination {
                // the path is currenly transvesed
                let mut path_num = 1;
                for node in &path {
                    self.cells[node.x][node.y] = CellType::Visited(path_num);
                    path_num += 1;
                }
                let duration = time.elapsed();
                // removing the destination_point from the path
                path.pop();
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

    #[doc = "BFS function for finding path"]
    #[doc = "The output is (path, time)"]
    pub fn bfs(&mut self, start_point: Point) -> Option<(Vec<Point>, f32)> {
        let time = Instant::now();
        let mut queue = VecDeque::<Point>::new();
        let mut visited = vec![vec![false; self.width]; self.height];
        let mut path = Vec::<Point>::new();

        queue.push_back(start_point);

        while let Some(current) = queue.pop_front() {
            let x = current.x;
            let y = current.y;

            visited[y][x] = true;
            path.push(Point { x: y, y: x });

            // removing the start_point from the path
            // doing it now so it could be done in O(1) exept O(N)
            if x == start_point.x && y == start_point.y {
                path.pop();
            }

            if self.cells[y][x] == CellType::Destination {
                // Path is found, mark the cells in the path
                let mut path_num = 1;
                for node in &path {
                    self.cells[node.x][node.y] = CellType::Visited(path_num);
                    path_num += 1;
                }
                let duration = time.elapsed();
                // removing the destination_point from the path
                path.pop();
                return Some((path, duration.as_secs_f32()));
            }

            let directions = [
                (0, -1), // Left
                (1, 0),  // Down
                (0, 1),  // Right
                (-1, 0), // Up
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
                    queue.push_back(new_point);
                    visited[new_point.y][new_point.x] = true; // Mark as visited
                }
            }
        }

        None // No path found
    }

    /// A* heuristic function (Manhatan distance)
    fn heuristic(start: Point, goal: Point) -> usize {
        ((start.x as isize - goal.x as isize).abs() + (start.y as isize - goal.y as isize).abs())
            as usize
    }

    fn construct_path(cells: HashMap<Point, Cell>, dest: Point) -> Vec<Point> {
        let mut path = vec![];
        let mut current = dest;
        while let Some(&node) = cells[&current].parent.as_ref() {
            current = node;
            path.push(current);
        }
        // remove the start
        path.pop();
        path.reverse();
        path
    }
    /// a_star path finding algorithm
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use path_algorithms::Grid;
    ///
    /// let mut grid = ;
    /// let result = grid.a_star(src, dest);
    /// assert_eq!(result, );
    /// assert_eq!(grid, );
    /// ```
    pub fn a_star(&mut self, src: Point, dest: Point) -> Option<(Vec<Point>, Vec<Point>, f32)> {
        let time = Instant::now();
        let mut open_list = BinaryHeap::<Reverse<Point>>::new();
        let mut closed_list = vec![vec![false; self.width]; self.height];
        let mut visited = Vec::<Point>::new();

        let mut cells = HashMap::<Point, Cell>::new();
        for i in 0..self.cells.len() {
            for j in 0..self.cells[0].len() {
                cells.insert(Point { x: j, y: i }, Cell::new());
            }
        }
        cells.insert(
            src,
            Cell {
                f: 0,
                g: 0,
                h: 0,
                parent: None,
            },
        );

        open_list.push(Reverse(src));
        while let Some(x) = open_list.pop() {
            let current = x.0;
            let Point { x, y } = current;
            closed_list[y][x] = true;

            if self.cells[x][y] == CellType::Destination {
                let mut path = Self::construct_path(cells, dest);
                // Path is found, mark the cells in the path
                let mut path_num = 1;
                for node in &path {
                    self.cells[node.y][node.x] = CellType::Visited(path_num);
                    path_num += 1;
                }
                let duration = time.elapsed();
                // removing the destination_point from the path
                path.pop();
                return Some((path, visited, duration.as_secs_f32()));
            }

            let directions = [
                (-1, 0), // Up
                (0, 1),  // Right
                (1, 0),  // Down
                (0, -1), // Left
            ];

            for (dx, dy) in &directions {
                let new_x = (x as isize + dx) as usize;
                let new_y = (y as isize + dy) as usize;

                if self.is_within_bounds(Point { x: new_x, y: new_y })
                    && self.cells[new_y][new_x] != CellType::Block
                {
                    let new_point = Point { x: new_x, y: new_y };
                    if !visited.contains(&Point { x: new_y, y: new_x }) {
                        visited.push(Point { x: new_y, y: new_x });
                    }

                    let tentative_g_score = cells.get(&current).unwrap().g + 1;
                    let tentative_h_score = Self::heuristic(Point { x: new_x, y: new_y }, dest);
                    let tentative_f_score = tentative_g_score + tentative_h_score;

                    if cells.get(&current).unwrap().g < cells.get(&new_point).unwrap().g {
                        cells.insert(
                            new_point,
                            Cell {
                                f: tentative_f_score,
                                h: tentative_h_score,
                                g: tentative_g_score,
                                parent: Some(current),
                            },
                        );
                        if !open_list.iter().any(|heap_item| heap_item.0 == new_point) {
                            open_list.push(Reverse(new_point));
                        }
                    }
                }
            }
        }

        None
    }
}
