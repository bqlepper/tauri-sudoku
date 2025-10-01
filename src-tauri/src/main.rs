// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://v1.tauri.app/v1/guides/features/command
#[tauri::command(rename_all = "snake_case")]
fn get_puzzle(cell_index: usize, user_input: &str) -> String {
    format!("Cell {} set to {}!!!", cell_index, user_input)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_puzzle])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
