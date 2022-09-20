#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use engine::{board::Board, client::ClientBoard};

mod engine;

#[tauri::command]
fn state(board: tauri::State<Board>) -> ClientBoard {
    board.to_client_board()
}

#[tauri::command]
fn execute(board: tauri::State<Board>) -> ClientBoard {
    board.to_client_board()
}

fn main() {
    tauri::Builder::default()
        .manage(Board::new())
        .invoke_handler(tauri::generate_handler![state, execute])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
