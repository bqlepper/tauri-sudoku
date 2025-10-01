// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

// Learn more about Tauri commands at https://v1.tauri.app/v1/guides/features/command
#[tauri::command(rename_all = "snake_case")]
fn get_puzzle(cell_index: usize, user_input: &str) -> String {
    format!("Cell {} set to {}!!!", cell_index, user_input)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"--cli".to_string()) {
        // Command-line only mode
        println!("Running in CLI mode...");
        // Add your CLI logic here
        println!("Example output: Hello from CLI mode!");
    } else {
        tauri::Builder::default()
            .invoke_handler(tauri::generate_handler![get_puzzle])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
}
