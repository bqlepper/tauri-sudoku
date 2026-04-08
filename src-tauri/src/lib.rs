use std::env;
use std::sync::Mutex;
use tauri::State;
use std::io::{self, Write};

use crate::sudoku_game::Game;

pub mod sudoku_game;
struct GameState(Mutex<Game>);

// For this user_change function, the cell_index is in the 0-80 range and user_input is in the 0-12 range.
//The user_input values must match up with values defined in src-ui/main.js:
//  0-9: Set the cell to that value (0 is used to clear a cell)
//  10: Clear the whole grid
//  11: Count the number of solutions remaining for the current grid and return that count to the caller.  If there are more than 100 solutions remaining, return "At least 100 solutions remaining" instead of the actual count.
//  12: Turn on debug mode which prints out the grid with all the possible values for each cell and the number of solutions remaining for each cell.
//  13: Turn off debug mode and return to normal grid display.

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

// Helper function to print available commands
fn print_help() {
    println!("Commands:");
    println!("  s <row> <col> <value>  - Set a value (row/col: 1-9, value: 1-9)");
    println!("  d <row> <col>          - Delete a value (row/col: 1-9)");
    println!("  c                      - Clear the entire grid");
    println!("  count                  - Count remaining solutions");
    println!("  debug on               - Turn on debug mode");
    println!("  debug off              - Turn off debug mode");
    println!("  h, help                - Show this help message");
    println!("  q, quit, exit          - Quit the program");
}

// New headless mode function
pub fn run_headless() {
    let mut game = Game::new();
    
    println!("=== Sudoku Headless Mode ===");
    print_help();
    println!();
    
    game.print_grid();
    
    loop {
        print!("\n> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        if input.is_empty() {
            continue;
        }
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        match parts[0] {
            "q" | "quit" | "exit" => {
                println!("Goodbye!");
                break;
            },
            "s" | "set" => {
                if parts.len() != 4 {
                    println!("Usage: s <row> <col> <value>");
                    continue;
                }
                
                let row: usize = match parts[1].parse::<usize>() {
                    Ok(r) if r >= 1 && r <= 9 => r - 1,
                    _ => {
                        println!("Row must be 1-9");
                        continue;
                    }
                };
                
                let col: usize = match parts[2].parse::<usize>() {
                    Ok(c) if c >= 1 && c <= 9 => c - 1,
                    _ => {
                        println!("Column must be 1-9");
                        continue;
                    }
                };
                
                let value: u8 = match parts[3].parse::<u8>() {
                    Ok(v) if v >= 1 && v <= 9 => v,
                    _ => {
                        println!("Value must be 1-9");
                        continue;
                    }
                };
                
                match game.user_set_value(row, col, value) {
                    Ok(_) => {
                        game.print_grid();
                    },
                    Err(msg) => {
                        println!("Error: {}", msg);
                    }
                }
            },
            "d" | "delete" => {
                if parts.len() != 3 {
                    println!("Usage: d <row> <col>");
                    continue;
                }
                
                let row: usize = match parts[1].parse::<usize>() {
                    Ok(r) if r >= 1 && r <= 9 => r - 1,
                    _ => {
                        println!("Row must be 1-9");
                        continue;
                    }
                };
                
                let col: usize = match parts[2].parse::<usize>() {
                    Ok(c) if c >= 1 && c <= 9 => c - 1,
                    _ => {
                        println!("Column must be 1-9");
                        continue;
                    }
                };
                
                game.user_delete_value(row, col);
                game.print_grid();
            },
            "c" | "clear" => {
                game.clear();
                game.print_grid();
            },
            "count" => {
                match game.count_solutions() {
                    Ok(count) => {
                        println!("Solutions remaining: {}", count);
                    },
                    Err(count) => {
                        if count == 0 {
                            println!("No solutions remaining");
                        } else {
                            println!("At least {} solutions remaining", count);
                        }
                    }
                }
            },
            "debug" => {
                if parts.len() != 2 {
                    println!("Usage: debug on|off");
                    continue;
                }
                
                match parts[1] {
                    "on" => {
                        game.set_debug(true);
                        game.print_grid();
                    },
                    "off" => {
                        game.set_debug(false);
                        game.print_grid();
                    },
                    _ => {
                        println!("Usage: debug on|off");
                    }
                }
            },
            "h" | "help" => {
                print_help();
            },
            _ => {
                println!("Unknown command. Type 'help' for available commands.");
                print_help();
            }
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(GameState(Mutex::new(Game::new()))) // Wrap Game in a Mutex
        .invoke_handler(tauri::generate_handler![user_change])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
