#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod engine;

use {engine::*, std::sync::Mutex};

#[tauri::command]
fn state(state: tauri::State<Mutex<Board>>) -> ClientBoard {
    let board = state.lock().unwrap();
    board.to_client_board()
}

#[tauri::command]
fn execute(state: tauri::State<Mutex<Board>>, r#move: Move) -> ClientBoard {
    let mut board = state.lock().unwrap();
    board.execute(r#move);
    board.to_client_board()
}

#[tauri::command]
fn reset(state: tauri::State<Mutex<Board>>) -> ClientBoard {
    let mut board = state.lock().unwrap();
    *board = Board::new();
    board.to_client_board()
}

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(Board::new()))
        .invoke_handler(tauri::generate_handler![state, execute, reset])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
