// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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
// Game: Creates a Grid.  Keeps track of the user sudoku entries and runs the brute force solver algorithm.
// Grid: A 9x9 array of cells in a sudoku puzzle.
// Cell: The smallest entity in a Sudoku puzzle that holds one value 1-9
// Box: A 3x3 array of cells.  There are 9 of these in a sudoku puzzle.
// Row: 9 horizontal cells.  There are 9 of these in a sudoku puzzle.
// Column: 9 vertical cells.  There are 9 of these in a sudoku puzzle.

// Learn more about Tauri commands at https://v1.tauri.app/v1/guides/features/command
#[tauri::command(rename_all = "snake_case")]
fn user_change(state: State<'_, GameState>, cell_index: usize, user_input: u8) -> String {
    // Lock the mutex to get mutable access to the Game
    let mut game = state.0.lock().unwrap();

    if user_input == 0 {
        game.user_delete_value(cell_index / 9, cell_index % 9);
        game.print_grid();
    } else {
        match game.user_set_value(cell_index / 9, cell_index % 9, user_input) {
            Ok(_) => {
                game.print_grid();
            },
            Err(user_msg) => {
                println!("Bad input: {user_msg}");
            }
        }
    }

    let grid_string = game.get_grid();
    println!("{}", grid_string);

    format!("Cell {} set to {}!!!", cell_index, user_input)
}

fn go_command_line() {
    let mut game: Game = Game::new();
    // This loop allows users to enter given puzzle values from the command line
    loop {
        println!("\nEnter row, column, and value separated by spaces (h for help):");

        // Read a line from standard input
        let mut input_line = String::new();
        match std::io::stdin().read_line(&mut input_line) {
            Ok(_) => {
                let input_numbers: Vec<&str> = input_line.trim().split_whitespace().collect();

                if input_numbers.len() == 1 {
                    match input_numbers[0].to_lowercase().as_str() {
                        "q" | "quit" => {
                            println!("Goodbye.");
                            break;
                        }
                        "d" | "debug" => {
                            game.set_debug(true);
                            game.print_grid();
                            continue;
                        }
                        "o" | "off" => {
                            game.set_debug(false);
                            game.print_grid();
                            continue;
                        }
                        "c" | "clear" => {
                            game.clear();
                            game.print_grid();
                            continue;
                        }
                        "h" | "help" => {
                            println!("\nDuring play the following keypresses are allowed:");
                            println!("\n  1-9              Enter sudoku puzzle clues");
                            println!("  del or backspace   Clear a puzzle clue that was already entered");
                            println!("  c (clear)          Clear the entire puzzle");
                            println!("  s (search)         Runs brute force solution search (can take a long time if too few clues provided)");
                            println!("  q (quit)           Exit the program (for command line only)");
                            println!("  d (debug)          Turn on debugging output on the command line");
                            println!("  o (off)            Turn off debugging output on the command line\n");
                            println!("For command line enter row column and value on one line separated by spaces with values 1-9 for all.\n");
                            println!("For GUI the mouse, arrow keys, and tab can be used to navigate the grid and use red window X to close.\n");
                            continue;
                        }
                        "s" | "search" => {
                            let result = game.start_search();
                            if let Err(out_message) = result {
                                println!("{out_message}");
                            }
                            continue;
                        }
                        "b" | "break" => { // BQL TODO this will not work until I figure out how to run the brut force search in a thread that can die when the user selects to stop the search.
                            game.stop_search();
                            continue;
                        }
                        _ => {
                            println!("Bad input, try again (or type 'help'): {}", input_line);
                            continue;
                        }
                    }
                }

                if input_numbers.len() < 3 {
                    println!("Bad input, try again (too few input vaules): {}", input_line);
                    continue;
                } else if input_numbers.len() > 3 {
                    println!("Warning: Ignoring extra input values");
                }

                let row = match input_numbers[0].parse::<usize>() {
                    Ok(row_num) => row_num,
                    Err(_) => 0,
                };

                let column = match input_numbers[1].parse::<usize>() {
                    Ok(column_num) => column_num,
                    Err(_) => 0,
                };

                if row < 1 || column < 1 {
                    println!("Row & column must be greater than zero");
                    continue;
                }

                let value = match input_numbers[2].parse::<u8>() {
                    Ok(value_num) => value_num,
                    Err(_) => 0,
                };

                match game.user_set_value(row-1, column-1, value) {
                    Ok(_) => {
                        game.print_grid();
                    },
                    Err(user_msg) => {
                        println!("Bad input: {user_msg}");
                        continue;
                    }
                }
            },
            Err(_) => {
                println!("Bad input (error reading line): {}", input_line);
                break;
            }
        }
    }
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
        go_command_line();
    } else {
        tauri::Builder::default()
            .manage(GameState(Mutex::new(Game::new()))) // Wrap Game in a Mutex
            .invoke_handler(tauri::generate_handler![user_change])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
        println!("Tauri app exited");
    }
}
