#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;

use engine::{board::Board, client::ClientBoard, r#move::Move};

use crate::engine::square::File;

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

#[tauri::command]
fn reset(state: tauri::State<Mutex<Board>>) -> ClientBoard {
	let mut board = state.lock().unwrap();
	*board = Board::new();
	board.to_client_board()
}

#[allow(unused_macros)]
macro_rules! execute {
    ($board:tt $square_1:tt $square_2:tt) => {
        $board.execute(
            $board
                .get_moves()
                .iter()
                .filter(|m| {
                    m.from == rs_tauri_chess::square!($square_1)
                        && m.to == rs_tauri_chess::square!($square_2)
                })
                .next()
                .unwrap()
                .clone(),
        );
    };
}

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(Board::new()))
        .invoke_handler(tauri::generate_handler![state, execute, reset])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
