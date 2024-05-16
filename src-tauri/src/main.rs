// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(special_module_name)]

mod lib;
#[allow(unused_imports)]
use lib::{ser_to_cell, ser_to_string, CellType, Grid, Point};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command(rename_all = "snake_case")]
async fn dfs_solve(arr: Vec<Vec<String>>, start: (usize, usize)) -> (Vec<(usize, usize)>, f32) {
    let mut grid = Grid::new(ser_to_cell(&arr));
    let start = Point {
        x: start.0,
        y: start.1,
    };
    if let Some(x) = grid.dfs(start) {
        return (x.0.iter().map(|i| i.as_tuple()).collect(), x.1);
    } else {
        return (Vec::new(), 0.0);
    }
}

#[tauri::command(rename_all = "snake_case")]
async fn bfs_solve(arr: Vec<Vec<String>>, start: (usize, usize)) -> (Vec<(usize, usize)>, f32) {
    let mut grid = Grid::new(ser_to_cell(&arr));
    let start = Point {
        x: start.0,
        y: start.1,
    };
    if let Some(x) = grid.bfs(start) {
        return (x.0.iter().map(|i| i.as_tuple()).collect(), x.1);
    } else {
        return (Vec::new(), 0.0);
    }
}

#[tauri::command(rename_all = "snake_case")]
/// async fn a_star_solve(arr: Vec<Vec<String>>) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, f32) {
async fn a_star_solve(
    arr: Vec<Vec<String>>,
    start: (usize, usize),
    dest: (usize, usize),
) -> Option<(Vec<(usize, usize)>, Vec<(usize, usize)>, f32)> {
    let mut grid = Grid::new(ser_to_cell(&arr));
    grid.swap_dim();
    let start = Point {
        x: start.0,
        y: start.1,
    };
    let dest = Point {
        x: dest.1,
        y: dest.0,
    };
    if let Some(x) = grid.a_star(start, dest) {
        return Some((
            x.0.iter().map(|i| i.as_tuple()).collect(),
            x.1.iter().map(|j| j.as_tuple()).collect(),
            x.2,
        ));
    } else {
        return None;
    }
}

#[tauri::command(rename_all = "snake_case")]
fn make_random_grid(width: usize, height: usize) -> Vec<Vec<String>> {
    let grid = Grid::random_grid(width, height);
    ser_to_string(&grid.cells)
}

fn main() {
    // let cells = vec![
    //     vec![CellType::Blank, CellType::Blank, CellType::Blank, CellType::Blank, CellType::Blank, CellType::Block,],
    //     vec![CellType::Blank, CellType::Blank, CellType::Blank, CellType::Block, CellType::Blank, CellType::Block,],
    //     vec![CellType::Blank, CellType::Blank, CellType::Blank, CellType::Blank, CellType::Block, CellType::Blank,],
    //     vec![CellType::Block, CellType::Start, CellType::Block, CellType::Blank, CellType::Blank, CellType::Destination,
    //     ],
    // ];
    // let mut grid = Grid::new(cells);
    // let a=grid.a_star(Point { x: 3, y: 1 },Point{ x: 3, y: 5 }).unwrap();
    // println!("the final grid is: {:?}", grid.cells);
    // println!("visited nodes: {:?}", a.1);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            dfs_solve,
            bfs_solve,
            a_star_solve,
            make_random_grid
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
