// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// This is the main rust program file.  It just launches the Tauri lib named in Cargo.toml and defined in lib.rs.

// However, I would like to make this main function take a command line option so I can run it in a
// debug mode that only uses command line user input and output for easy testing and debugging in a
// headless environment.

// -- Terminology Definitions --
// Game: Creates a Grid.  Keeps track of the user sudoku entries and has a recursive solution solver.
// Grid: A 9x9 array of cells in a sudoku puzzle.
// Cell: The smallest entity in a Sudoku puzzle that holds one value 1-9
// Box: A 3x3 array of cells.  There are 9 of these in a sudoku puzzle.
// Row: 9 horizontal cells.  There are 9 of these in a sudoku puzzle.
// Column: 9 vertical cells.  There are 9 of these in a sudoku puzzle.

fn main() {
    tauri2_sudoku_lib::run()
}
