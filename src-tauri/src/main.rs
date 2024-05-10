// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(special_module_name)]

mod lib;
use lib::{ser_to_cell, ser_to_string, Grid, Point};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command(rename_all = "snake_case")]
async fn test_vec(arr: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let grid_data = ser_to_cell(&arr);
    let mut grid = Grid::new(grid_data);
    let start = Point { x: 0, y: 0 };
    grid.solve(start);

    let out = ser_to_string(&grid.cells);
    out
}

fn main() {
    // let cells = vec![
    //     vec![CellType::Start,CellType::Blank,CellType::Blank,CellType::Block,], //asdasd
    //     vec![CellType::Block,CellType::Blank,CellType::Blank,CellType::Blank,], //asdasd
    //     vec![CellType::Blank,CellType::Blank,CellType::Block,CellType::Blank,], //asdas
    //     vec![CellType::Blank,CellType::Blank,CellType::Destination,CellType::Block,], //asdasd
    // ];
    // println!("vec in rust:\n{:?}", cells);

    // let mut grid = Grid::new(cells);

    // let start_point = Point { x: 0, y: 0 };
    // grid.solve(start_point);
    // println!("{:?}", grid.cells);
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, test_vec])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
