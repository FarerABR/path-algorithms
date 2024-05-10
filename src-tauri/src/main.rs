// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(special_module_name)]

mod lib;
use lib::{ser_to_cell, CellType, Grid, Point};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command(rename_all = "snake_case")]
async fn dfs_solve(arr: Vec<Vec<String>>) -> (Vec<(usize, usize)>, f32) {
    let grid_data = ser_to_cell(&arr);
    let mut grid = Grid::new(grid_data);
    let start = Point { x: 0, y: 0 };
    if let Some(x) = grid.dfs(start) {
        return (x.0.iter().map(|i| i.as_tuple()).collect(), x.1);
    } else {
        return (Vec::new(), 0.0);
    }
}

#[tauri::command(rename_all = "snake_case")]
async fn bfs_solve(arr: Vec<Vec<String>>) -> (Vec<(usize, usize)>, f32) {
    let grid_data = ser_to_cell(&arr);
    let mut grid = Grid::new(grid_data);
    let start = Point { x: 0, y: 0 };
    if let Some(x) = grid.bfs(start) {
        return (x.0.iter().map(|i| i.as_tuple()).collect(), x.1);
    } else {
        return (Vec::new(), 0.0);
    }
}

fn main() {
    let cells = vec![
        vec![
            CellType::Start,
            CellType::Blank,
            CellType::Blank,
            CellType::Block,
        ],
        vec![
            CellType::Block,
            CellType::Blank,
            CellType::Blank,
            CellType::Block,
        ],
        vec![
            CellType::Block,
            CellType::Block,
            CellType::Blank,
            CellType::Blank,
        ],
        vec![
            CellType::Block,
            CellType::Block,
            CellType::Destination,
            CellType::Blank,
        ],
    ];

    let mut grid = Grid::new(cells);

    let start_point = Point { x: 0, y: 0 };

    if let Some((path, duration)) = grid.bfs(start_point) {
        println!("Path found:");
        for point in &path {
            println!("({}, {})", point.x, point.y);
        }
        println!("Elapsed time: {:.3} seconds", duration);

        println!("{:?}", grid.cells);
    } else {
        println!("No path found.");
    }
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, dfs_solve, bfs_solve])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
