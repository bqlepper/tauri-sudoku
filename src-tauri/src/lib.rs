use std::env;
use std::io::{self, Write};
use std::sync::Mutex;
use tauri::State;

use crate::sudoku_game::Game;
use crate::sudoku_game::sudoku_constants::{GRID_SIDE, VALUE_MAX, VALUE_MIN};

pub mod sudoku_game;
pub mod test_harness;
struct GameState(Mutex<Game>);

const CELL_COUNT: usize = GRID_SIDE * GRID_SIDE;

// Public API
// New headless mode function
pub fn run_headless() {
    let mut game = Game::new();

    println!("=== Sudoku Headless Mode ===");
    print_help();
    println!();

    game.print_grid();

    loop {
        print!("\n> ");
        if let Err(error) = io::stdout().flush() {
            eprintln!("Output error: {error}");
            break;
        }

        let mut input = String::new();
        if let Err(error) = io::stdin().read_line(&mut input) {
            eprintln!("Input error: {error}");
            break;
        }
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();

        match parts[0] {
            "q" | "quit" | "exit" => {
                println!("Goodbye!");
                break;
            }
            "s" | "set" => {
                if parts.len() != 4 {
                    println!("Usage: s <row> <col> <value>");
                    continue;
                }

                let row = match parse_one_based(parts[1], "Row") {
                    Ok(row) => row,
                    Err(message) => {
                        println!("{message}");
                        continue;
                    }
                };

                let col = match parse_one_based(parts[2], "Column") {
                    Ok(column) => column,
                    Err(message) => {
                        println!("{message}");
                        continue;
                    }
                };

                let value = match parse_value(parts[3]) {
                    Ok(value) => value,
                    Err(message) => {
                        println!("{message}");
                        continue;
                    }
                };

                match game.user_set_value(row, col, value) {
                    Ok(_) => {
                        game.print_grid();
                    }
                    Err(msg) => {
                        println!("Error: {}", msg);
                    }
                }
            }
            "d" | "delete" => {
                if parts.len() != 3 {
                    println!("Usage: d <row> <col>");
                    continue;
                }

                let row = match parse_one_based(parts[1], "Row") {
                    Ok(row) => row,
                    Err(message) => {
                        println!("{message}");
                        continue;
                    }
                };

                let col = match parse_one_based(parts[2], "Column") {
                    Ok(column) => column,
                    Err(message) => {
                        println!("{message}");
                        continue;
                    }
                };

                match game.user_delete_value(row, col) {
                    Ok(_) => {
                        game.print_grid();
                    }
                    Err(msg) => {
                        println!("Error: {}", msg);
                    }
                }
            }
            "c" | "clear" => {
                game.clear();
                game.print_grid();
            }
            "count" => match game.count_solutions() {
                Ok(count) => {
                    println!("Solutions remaining: {}", count);
                }
                Err(count) => {
                    if count == 0 {
                        println!("No solutions remaining");
                    } else {
                        println!("At least {} solutions remaining", count);
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
                    }
                    "off" => {
                        game.set_debug(false);
                        game.print_grid();
                    }
                    _ => {
                        println!("Usage: debug on|off");
                    }
                }
            }
            "test" => {
                run_tests();
            }
            "h" | "help" => {
                print_help();
            }
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

// Private utility functions
// For this user_change function, the cell_index is in the 0-80 range and user_input is in the 0-13 range.
//The user_input values must match up with values defined in src-ui/main.js:
//  0-9: Set the cell to that value (0 is used to clear a cell)
//  10: Clear the whole grid
//  11: Count the number of solutions remaining for the current grid and return that count to the caller. If there are 50 or more solutions remaining, return "At least 50 solutions remaining" instead of the exact count.
//  12: Turn on debug mode which prints out the grid with all the possible values for each cell and the number of solutions remaining for each cell.
//  13: Turn off debug mode and return to normal grid display.

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command(rename_all = "snake_case")]
fn user_change(state: State<'_, GameState>, cell_index: usize, user_input: u8) -> String {
    // Lock the mutex to get mutable access to the Game
    let mut game = match state.0.lock() {
        Ok(game) => game,
        Err(_) => return "Bad input: Internal game state lock failed".to_string(),
    };
    run_user_change(&mut game, cell_index, user_input)
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
    println!("  test                   - Run all tests from test directory");
    println!("  h, help                - Show this help message");
    println!("  q, quit, exit          - Quit the program");
}

// Run the test harness
fn run_tests() {
    use std::path::Path;

    // Try to find the test directory
    let test_paths = [
        "../../test",           // From cargo run location
        "../test",              // Alternative
        "test",                 // If running from project root
    ];

    let test_dir = test_paths
        .iter()
        .map(Path::new)
        .find(|p| p.exists())
        .unwrap_or_else(|| {
            println!("Warning: Could not find test directory. Trying '../../test'");
            Path::new("../../test")
        });

    println!("\n=== Running Test Suite ===");
    println!("Test directory: {}\n", test_dir.display());

    let results = test_harness::run_all_tests(test_dir);

    let mut passed = 0;
    let mut failed = 0;

    for result in &results {
        if result.success {
            println!("[PASS] {} - {}", result.test_name, result.message);
            passed += 1;
        } else {
            println!("[FAIL] {} - {}", result.test_name, result.message);
            failed += 1;
        }
    }

    println!("\n=== Test Summary ===");
    println!("Total: {}", passed + failed);
    println!("Passed: {}", passed);
    println!("Failed: {}", failed);

    if failed == 0 {
        println!("\nAll tests passed!");
    }
}

fn run_user_change(game: &mut Game, cell_index: usize, user_input: u8) -> String {
    if matches!(user_input, 0 | VALUE_MIN..=VALUE_MAX) {
        let (row, column) = match cell_position_from_index(cell_index) {
            Ok(position) => position,
            Err(message) => return format!("Bad input: {message}"),
        };

        if user_input == 0 {
            match game.user_delete_value(row, column) {
                Ok(_) => {
                    game.print_grid();
                    return game.get_grid();
                }
                Err(user_msg) => {
                    println!("Bad input: {user_msg}");
                    return format!("Bad input: {user_msg}");
                }
            }
        }

        match game.user_set_value(row, column, user_input) {
            Ok(_) => {
                game.print_grid();
                return game.get_grid();
            }
            Err(user_msg) => {
                println!("Bad input: {user_msg}");
                return format!("Bad input: {user_msg}");
            }
        }
    }

    match user_input {
        10 => {
            game.clear();
            game.print_grid();
            game.get_grid()
        }
        11 => match game.count_solutions() {
            Ok(count) => format!("Solutions remaining: {}", count),
            Err(count) if count == 0 => "No solutions remaining".to_string(),
            Err(count) => format!("At least {} solutions remaining", count),
        },
        12 => {
            game.set_debug(true);
            game.print_grid();
            game.get_grid()
        }
        13 => {
            game.set_debug(false);
            game.print_grid();
            game.get_grid()
        }
        _ => format!("Bad input: Invalid command {}", user_input),
    }
}

fn parse_one_based(raw: &str, label: &str) -> Result<usize, String> {
    match raw.parse::<usize>() {
        Ok(value) if (1..=GRID_SIDE).contains(&value) => Ok(value - 1),
        _ => Err(format!("{label} must be 1-{}", GRID_SIDE)),
    }
}

fn parse_value(raw: &str) -> Result<u8, String> {
    match raw.parse::<u8>() {
        Ok(value) if (VALUE_MIN..=VALUE_MAX).contains(&value) => Ok(value),
        _ => Err(format!("Value must be {}-{}", VALUE_MIN, VALUE_MAX)),
    }
}

fn cell_position_from_index(cell_index: usize) -> Result<(usize, usize), String> {
    if cell_index >= CELL_COUNT {
        return Err(format!(
            "Invalid cell index {} (expected 0-{})",
            cell_index,
            CELL_COUNT - 1
        ));
    }
    Ok((cell_index / GRID_SIDE, cell_index % GRID_SIDE))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_one_based_rejects_out_of_range_values() {
        assert_eq!(parse_one_based("0", "Row"), Err("Row must be 1-9".to_string()));
        assert_eq!(
            parse_one_based("10", "Column"),
            Err("Column must be 1-9".to_string())
        );
        assert_eq!(
            parse_one_based("abc", "Row"),
            Err("Row must be 1-9".to_string())
        );
    }

    #[test]
    fn cell_position_from_index_checks_bounds() {
        assert_eq!(cell_position_from_index(0), Ok((0, 0)));
        assert_eq!(cell_position_from_index(80), Ok((8, 8)));
        assert_eq!(
            cell_position_from_index(81),
            Err("Invalid cell index 81 (expected 0-80)".to_string())
        );
    }

    #[test]
    fn run_user_change_rejects_invalid_cell_index_for_value_actions() {
        let mut game = Game::new();
        let error = run_user_change(&mut game, 81, 1);
        assert_eq!(error, "Bad input: Invalid cell index 81 (expected 0-80)");

        let delete_error = run_user_change(&mut game, 100, 0);
        assert_eq!(
            delete_error,
            "Bad input: Invalid cell index 100 (expected 0-80)"
        );
    }

    #[test]
    fn run_user_change_rejects_unknown_command_values() {
        let mut game = Game::new();
        assert_eq!(
            run_user_change(&mut game, 0, 99),
            "Bad input: Invalid command 99"
        );
    }
}
