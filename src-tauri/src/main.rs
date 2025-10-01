// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use sudoku_cmd_line::Puzzled;

pub mod sudoku_game;
pub mod sudoku_cmd_line;

// -- Design and Terminology Definitions --
// This is the main rust program file.  It handles any command line options when starting the program.
// It creates a Puzzle or a Puzzled based on user command line input.
// Puzzle: Creates a sudoku Game and handles user interaction with a GUI for the sudoku Game.
// Puzzled: Creates a sudoku Game and handles user interaction with command-line-only used for debug.
// Game: Creates a Grid.  Keeps track of the user sudoku entries and runs the brute force solver algorithm.
// Grid: A 9x9 array of cells in a sudoku puzzle.
// Cell: The smallest entity in a Sudoku puzzle that holds one value 1-9
// Box: A 3x3 array of cells.  There are 9 of these in a sudoku puzzle.
// Row: 9 horizontal cells.  There are 9 of these in a sudoku puzzle.
// Column: 9 vertical cells.  There are 9 of these in a sudoku puzzle.

// Learn more about Tauri commands at https://v1.tauri.app/v1/guides/features/command
#[tauri::command(rename_all = "snake_case")]
fn get_puzzle(cell_index: usize, user_input: &str) -> String {
    format!("Cell {} set to {}!!!", cell_index, user_input)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match env::current_dir() {
        Ok(pb) => println!("The current directory is {}", pb.display()),
        Err(_) => println!("Error getting current working directory!")
    }

    if args.contains(&"--cli".to_string()) {
        // This code implements a command line version of the UI that is good for debugging
        // It allows the users to interactively enter given puzzle values with command line
        // But also offers a debug view of all the values to see what potential values are remaining
        let mut debug_puzzle = Puzzled::new();
        debug_puzzle.go();
    } else {
        tauri::Builder::default()
            .invoke_handler(tauri::generate_handler![get_puzzle])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
}
