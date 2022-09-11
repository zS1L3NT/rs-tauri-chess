use engine::board::Board;

mod engine;
// #![cfg_attr(
//     all(not(debug_assertions), target_os = "windows"),
//     windows_subsystem = "windows"
// )]

#[tauri::command]
fn get_possible_moves() {}

fn main() {
    Board::new();
    // tauri::Builder::default()
    //     .invoke_handler(tauri::generate_handler![])
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");
}
