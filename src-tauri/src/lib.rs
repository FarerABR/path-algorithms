use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, VecDeque},
    hash::Hash,
    time::Instant,
};

use rand::Rng;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
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
#[allow(dead_code)]
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
    pub fn swap_dim(&mut self) {
        let tmp = self.height;
        self.height = self.width;
        self.width = tmp;
    }

    pub fn random_grid(width: usize, height: usize) -> Self {
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
        while placed < block_num {
            let (x, y) = (
                rng.gen_range(0..(width - 1)),
                rng.gen_range(0..(height - 1)),
            );
            if grid[x][y] == CellType::Blank {
                grid[x][y] = CellType::Block;
                placed += 1;
            }
        }

        // println!("{:?}", grid);
        Self {
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
        let mut visited = vec![vec![false; self.width]; self.height];
        let mut path = Vec::<Point>::new();

        stack.push(Point {
            x: start_point.y,
            y: start_point.x,
        });

        while let Some(current) = stack.pop() {
            let x = current.x;
            let y = current.y;

            visited[y][x] = true;
            if !path.contains(&Point { x: y, y: x }) {
                path.push(Point { x: y, y: x });
            }

            // removing the start_point from the path
            // doing it now so it could be done in O(1) instead of O(N)
            if x == start_point.y && y == start_point.x {
                path.pop();
            }

            // reached destination
            if self.cells[y][x] == CellType::Destination {
                // the path is currenly transvesed
                // removing the destination_point from the path
                path.pop();
                // let mut path_num = 1;
                // for node in &path {
                //     self.cells[node.x][node.y] = CellType::Visited(path_num);
                //     path_num += 1;
                // }
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
                if self.is_within_bounds(new_point) && !visited[new_point.y][new_point.x] {
                    if self.cells[new_point.y][new_point.x] == CellType::Blank
                        || self.cells[new_point.y][new_point.x] == CellType::Destination
                    {
                        stack.push(new_point)
                    }
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

        queue.push_back(Point {
            x: start_point.y,
            y: start_point.x,
        });

        while let Some(current) = queue.pop_front() {
            let x = current.x;
            let y = current.y;

            visited[y][x] = true;
            if !path.contains(&Point { x: y, y: x }) {
                path.push(Point { x: y, y: x });
            }

            // removing the start_point from the path
            // doing it now so it could be done in O(1) exept O(N)
            if x == start_point.y && y == start_point.x {
                path.pop();
            }

            if self.cells[y][x] == CellType::Destination {
                // Path is found, mark the cells in the path

                // removing the destination_point from the path
                path.pop();
                // let mut path_num = 1;
                // for node in &path {
                //     self.cells[node.x][node.y] = CellType::Visited(path_num);
                //     path_num += 1;
                // }
                let duration = time.elapsed();
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

                if self.is_within_bounds(new_point) && !visited[new_point.y][new_point.x] {
                    if self.cells[new_point.y][new_point.x] == CellType::Blank
                        || self.cells[new_point.y][new_point.x] == CellType::Destination
                    {
                        queue.push_back(new_point);
                        visited[new_point.y][new_point.x] = true; // Mark as visited
                    }
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
        let mut current = Point {
            x: dest.y,
            y: dest.x,
        };
        while let Some(node) = cells[&current].parent {
            current = node;
            path.push(Point {
                x: current.x,
                y: current.y,
            });
        }
        // remove the start
        path.pop();
        path.reverse();
        path
    }
    /// a_star path finding algorithm
    pub fn a_star(&mut self, src: Point, dest: Point) -> Option<(Vec<Point>, Vec<Point>, f32)> {
        let time = Instant::now();
        // let mut open_list = Vec::<(usize, Point)>::new();
        let mut open_list = BinaryHeap::<Reverse<(usize, Point)>>::new();
        let mut visited = Vec::<Point>::new();

        let mut cells = HashMap::<Point, Cell>::new();
        for i in 0..self.cells.len() {
            for j in 0..self.cells[0].len() {
                cells.insert(Point { x: i, y: j }, Cell::new());
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
        open_list.push(Reverse((
            Self::heuristic(
                src,
                Point {
                    x: dest.y,
                    y: dest.x,
                },
            ),
            src,
        )));
        while let Some(x) = open_list.pop() {
            // while open_list.len() > 0 {
            // let current = open_list.remove(0);
            let current = x.0;
            let Point { x, y } = current.1;
            if !visited.contains(&Point { x: x, y: y }) {
                visited.push(Point { x: x, y: y });
            }

            if self.cells[x][y] == CellType::Destination {
                // Path is found, mark the cells in the path
                let path = Self::construct_path(cells, dest);
                // removing start and destination from the visited
                if visited.contains(&src) {
                    visited.remove(visited.iter().position(|&i| i == src).unwrap());
                }
                if visited.contains(&Point {
                    x: dest.y,
                    y: dest.x,
                }) {
                    visited.remove(
                        visited
                            .iter()
                            .position(|&i| {
                                i == Point {
                                    x: dest.y,
                                    y: dest.x,
                                }
                            })
                            .unwrap(),
                    );
                }
                let duration = time.elapsed();
                return Some((path, visited, duration.as_secs_f32()));
            }
            let mut directions = [
                (-1, 0), // Up
                (0, 1),  // Right
                (1, 0),  // Down
                (0, -1), // Left
            ];
            let mut heu = Vec::new();
            for i in directions {
                heu.push(Self::heuristic(
                    Point {
                        x: (x as isize + i.0) as usize,
                        y: (y as isize + i.1) as usize,
                    },
                    Point {
                        x: dest.y,
                        y: dest.x,
                    },
                ));
            }
            for i in 0..heu.len() {
                // Assume the current index has the minimum value
                let mut min_index = i;

                // Find the index of the smallest element in the remaining unsorted portion
                for j in (i + 1)..heu.len() {
                    if heu[j] < heu[min_index] {
                        min_index = j;
                    }
                }

                // Swap the current element with the smallest element found
                if min_index != i {
                    heu.swap(i, min_index);
                    directions.swap(i, min_index);
                }
            }
            for (dx, dy) in &directions {
                let new_x = (x as isize + dx) as usize;
                let new_y = (y as isize + dy) as usize;

                if self.is_within_bounds(Point { x: new_x, y: new_y })
                    && (self.cells[new_x][new_y] == CellType::Blank
                        || self.cells[new_x][new_y] == CellType::Destination)
                {
                    let new_point = Point { x: new_x, y: new_y };

                    let tentative_g_score =
                        usize::wrapping_add(cells.get(&current.1).unwrap().g, 1);
                    let tentative_h_score = Self::heuristic(
                        Point { x: new_x, y: new_y },
                        Point {
                            x: dest.y,
                            y: dest.x,
                        },
                    );
                    let tentative_f_score = tentative_g_score + tentative_h_score;

                    if cells.get(&new_point).unwrap().g == usize::MAX
                        || tentative_g_score < cells.get(&new_point).unwrap().g
                    {
                        cells.insert(
                            new_point,
                            Cell {
                                f: tentative_f_score,
                                h: tentative_h_score,
                                g: tentative_g_score,
                                parent: Some(current.1),
                            },
                        );

                        if !open_list.iter().any(|x| x.0 .1 == new_point) {
                            // if !open_list.iter().any(|x| x.1 == new_point) {
                            // open_list.push((tentative_f_score, new_point));
                            // open_list.sort_by_key(|&(f_score, _)| f_score);
                            open_list.push(Reverse((tentative_f_score, new_point)));
                        }
                    }
                }
            }
        }

        None
    }
}
