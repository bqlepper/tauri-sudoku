use std::env;
use std::sync::Mutex;
use tauri::State;

use crate::sudoku_game::Game;

pub mod sudoku_game;
struct GameState(Mutex<Game>);

// -- Design and Terminology Definitions --
// This is the main rust program file.  It handles any command line options when starting the program.
// It creates a Puzzle or a Puzzled based on user command line input.
// Puzzle: Creates a sudoku Game and handles user interaction with a GUI for the sudoku Game.
// Puzzled: Creates a sudoku Game and handles user interaction with command-line-only used for debug.
// Game: Creates a Grid.  Keeps track of the user sudoku entries and has a recursive solution solver.
// Grid: A 9x9 array of cells in a sudoku puzzle.
// Cell: The smallest entity in a Sudoku puzzle that holds one value 1-9
// Box: A 3x3 array of cells.  There are 9 of these in a sudoku puzzle.
// Row: 9 horizontal cells.  There are 9 of these in a sudoku puzzle.
// Column: 9 vertical cells.  There are 9 of these in a sudoku puzzle.

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command(rename_all = "snake_case")]
fn user_change(state: State<'_, GameState>, cell_index: usize, user_input: u8) -> String {
    // Lock the mutex to get mutable access to the Game
    let mut game = state.0.lock().unwrap();

    if user_input == 0 {
        game.user_delete_value(cell_index / 9, cell_index % 9);
        game.print_grid();
    } else if user_input == 10 {
        game.clear();
        game.print_grid();
    } else if user_input == 11 {
        match game.count_solutions() {
            Ok(count) => {
                return format!("Solutions remaining: {}", count);
            },
            Err(count) => {
                if count == 0 {
                    return "No solutions remaining".to_string();
                }
                return format!("At least {} solutions remaining", count);
            }
        }
    } else if user_input == 12 {
        game.set_debug(true);
        game.print_grid();
    } else if user_input == 13 {
        game.set_debug(false);
        game.print_grid();
    } else {
        match game.user_set_value(cell_index / 9, cell_index % 9, user_input) {
            Ok(_) => {
                game.print_grid();
            },
            Err(user_msg) => {
                println!("Bad input: {user_msg}");
                return format!("Bad input: {user_msg}");
            }
        }
    }

    game.get_grid()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .manage(GameState(Mutex::new(Game::new()))) // Wrap Game in a Mutex
        .invoke_handler(tauri::generate_handler![user_change])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
