// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// This is the main rust program file.  It just launches the Tauri lib named in Cargo.toml and defined in lib.rs.
// If --headless is present on the command line, it is run without a Tauri UI with only command line input/output.

// -- Terminology Definitions --
// Game: Creates a Grid.  Keeps track of the user sudoku entries and has a recursive solution solver.
// Grid: A 9x9 array of cells in a sudoku puzzle.
// Cell: The smallest entity in a Sudoku puzzle that holds one value 1-9
// Box: A 3x3 array of cells.  There are 9 of these in a sudoku puzzle.
// Row: 9 horizontal cells.  There are 9 of these in a sudoku puzzle.
// Column: 9 vertical cells.  There are 9 of these in a sudoku puzzle.

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Check if --headless flag is present
    if args.iter().any(|arg| arg == "--headless" || arg == "-h") {
        tauri2_sudoku_lib::run_headless();
    } else {
        tauri2_sudoku_lib::run();
    }
}
