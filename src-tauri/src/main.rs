// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://v1.tauri.app/v1/guides/features/command
#[tauri::command]
fn user_set_cell(cell_number: &str) -> String {
    println!("Hey there");
    println!("User set cell {} to OOPS!", cell_number);
    format!("User set cell {} to OOPS!", cell_number)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![user_set_cell])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
