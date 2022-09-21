#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;

use engine::{board::Board, client::ClientBoard, r#move::Move};

mod engine;

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

fn main() {
	println!("{:#?}", Board::new().get_moves());
    // tauri::Builder::default()
    //     .manage(Mutex::new(Board::new()))
    //     .invoke_handler(tauri::generate_handler![state, execute])
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");
}
