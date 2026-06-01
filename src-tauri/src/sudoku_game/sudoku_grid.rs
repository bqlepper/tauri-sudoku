use colored::*;
use sudoku_cell::Cell;
use super::exact_cover::{Assignment, ExactCoverAnalyzer, ExactCoverError};
use super::sudoku_constants::{BOX_SIDE as SUDOKU_BOX_SIDE, GRID_SIDE, VALUE_MAX};

pub mod sudoku_cell;
pub mod sudoku_grid_trials;

pub(super) const GRID_SIZE: usize = GRID_SIDE;
pub(super) const BOX_SIDE: usize = SUDOKU_BOX_SIDE;

// Grid has all the sudoku rule checking functions and prints out the grid to the command line
#[derive(Clone)]
pub(super) struct Grid {
    grid: [[Cell; GRID_SIZE]; GRID_SIZE],
    debug_output: bool, // Allows the print out of all possible values of each cell to the screen for debug purposes
}

impl Grid {
    pub(super) fn new() -> Grid {
        Grid {
            grid: [[Cell::new(); GRID_SIZE]; GRID_SIZE],
            debug_output: false,
        }
    }

    // Clears out all cells.
    pub(super) fn clear(&mut self) {
        self.grid.iter_mut().flat_map(|row| row.iter_mut()).for_each(|cell| {
            cell.clear();
        });
    }

    // returns true if every cell is solved, false otherwise.
    pub(super) fn is_solved(&self) -> bool {
        self.grid.iter().flat_map(|row| row.iter()).all(|cell| {
            cell.is_solved()
        })
    }

    pub(super) fn set_debug(&mut self, on: bool) { self.debug_output = on; }

    pub(super) fn lock_set_by_user(&mut self, row: usize, column:usize) { self.grid[row][column].lock_set_by_user(); }

    // Get the grid as a JSON string
    pub(super) fn get_grid(&self) -> String {
        use serde_json::json;
        let mut cells = Vec::new();
        for row in 0..GRID_SIZE {
            for column in 0..GRID_SIZE {
                let is_user_set = self.grid[row][column].is_set_by_user();
                let value = match self.grid[row][column].get_value() {
                    Err(_) => -1, // Error state
                    Ok(0) => 0,   // Empty
                    Ok(v) => v as i32,
                };

                cells.push(json!({
                    "value": value,
                    "isUserSet": is_user_set,
                    "isSolved": !is_user_set && value > 0,
                    "isError": value == -1
                }));
            }
        }

        let grid_data = json!({
            "cells": cells,
            "isSolved": self.is_solved()
        });

        grid_data.to_string()
    }

    pub(super) fn apply_exact_cover_checks(&mut self) -> Result<(), String> {
        let analyzer = self.exact_cover_analyzer()?;
        if analyzer.has_contradiction() {
            return Err("Exact cover contradiction detected".to_string());
        }

        let candidate_masks = analyzer
            .candidate_masks()
            .map_err(Self::exact_cover_error_message)?;

        for row in 0..GRID_SIZE {
            for column in 0..GRID_SIZE {
                let was_user_set = self.grid[row][column].is_set_by_user();
                let cell_mask = candidate_masks[row][column];
                if cell_mask == 0 {
                    return Err(format!(
                        "Exact cover produced no candidates for row {} column {}",
                        row + 1,
                        column + 1
                    ));
                }

                self.grid[row][column].set_values(cell_mask).map_err(|_| {
                    format!(
                        "Exact cover produced invalid candidates for row {} column {}",
                        row + 1,
                        column + 1
                    )
                })?;

                if was_user_set {
                    self.grid[row][column].lock_set_by_user();
                }
            }
        }

        Ok(())
    }

    pub(super) fn count_exact_cover_solutions_with_cap(&self, cap: usize) -> Result<usize, String> {
        let analyzer = self.exact_cover_analyzer()?;
        analyzer
            .count_solutions_with_cap(cap)
            .map_err(Self::exact_cover_error_message)
    }

    pub(super) fn has_direct_conflict(&self, row: usize, column: usize, value: u8) -> bool {
        for check_column in 0..GRID_SIZE {
            if check_column == column || !self.grid[row][check_column].is_solved() {
                continue;
            }

            if let Ok(existing) = self.grid[row][check_column].get_value() {
                if existing == value {
                    return true;
                }
            }
        }

        for check_row in 0..GRID_SIZE {
            if check_row == row || !self.grid[check_row][column].is_solved() {
                continue;
            }

            if let Ok(existing) = self.grid[check_row][column].get_value() {
                if existing == value {
                    return true;
                }
            }
        }

        let (start_row, end_row, start_column, end_column) = get_box_dimensions(row, column);
        for check_row in start_row..end_row {
            for check_column in start_column..end_column {
                if (check_row == row && check_column == column)
                    || !self.grid[check_row][check_column].is_solved()
                {
                    continue;
                }

                if let Ok(existing) = self.grid[check_row][check_column].get_value() {
                    if existing == value {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub(super) fn user_delete_value(&mut self, row: usize, column: usize) -> Result<(), String> {
        if row >= GRID_SIZE {
            return Err(format!("Invalid row {}", row + 1));
        }
        if column >= GRID_SIZE {
            return Err(format!("Invalid column {}", column + 1));
        }

        // Don't let the user delete a value that they have not already set
        if self.grid[row][column].is_set_by_user() {
            self.grid[row][column].clear();  //Clear the cells user settings
            let replay_user_settings = self.get_user_settings()?; // Save all the other user settings
            self.clear(); // Clear the whole grid
            for user_setting in replay_user_settings { // Replay all the other user settings
                self.set_value(user_setting.0, user_setting.1, user_setting.2).map_err(|error| {
                    format!(
                        "Unexpected error resetting row {} column {} value {}: {}",
                        user_setting.0 + 1,
                        user_setting.1 + 1,
                        user_setting.2,
                        error
                    )
                })?;
                self.grid[user_setting.0][user_setting.1].lock_set_by_user();
            }

            if !self.is_solvable() {
                return Err("Unexpected unsolvable grid after resetting a user value".to_string());
            }
            self.run_exact_cover_checks();
        }
        Ok(())
    }

    // Returns false if this value was already set, otherwise returns true to indicate a change
    pub(super) fn set_value(&mut self, row: usize, column: usize, value: u8) -> Result<bool, String> {
        if row >= GRID_SIZE {
            return Err(format!("Invalid row {}", row+1));
        }
        if column >= GRID_SIZE {
            return Err(format!("Invalid column {}", column+1));
        }
        if value == 0 || value > VALUE_MAX {
            return Err(format!("Invalid value {value}"));
        }

        match self.grid[row][column].set_value(value) {
            Err(()) => return Err(format!("{value} is not a valid entry for row {} column {}", row+1, column+1)),
            Ok(changed) => return Ok(changed),
        }
    }

    // Prints the current state of the Sudoku grid to the command line
    pub(super) fn print_grid(&self) {
        println!();
        if self.debug_output {
            for (row, row_arr) in self.grid.iter().enumerate() {
                if row != 0 && row%BOX_SIDE == 0
                {
                    println!("{}", "============+===========+===========".bright_cyan());
                }

                for row_index in 0..BOX_SIDE {
                    for (column, val) in row_arr.iter().enumerate() {
                        if column != 0 && column%BOX_SIDE == 0
                        {
                            print!("{}", "|".bright_cyan());
                        } else {
                            print!(" ");
                        }
                        let mut chars: Vec<char> = vec![' ', ' ', ' '];
                        match val.get_value() {
                            Err(_) => print!("{}", "XXX".bright_red()),
                            Ok(value) => {
                                if self.grid[row][column].is_set_by_user() {
                                    if row_index == 0 {
                                        if value == 1 { chars[0] = '1' };
                                        if value == 2 { chars[1] = '2' };
                                        if value == 3 { chars[2] = '3' };
                                    } else if row_index == 1 {
                                        if value == 4 { chars[0] = '4' };
                                        if value == 5 { chars[1] = '5' };
                                        if value == 6 { chars[2] = '6' };
                                    } else if row_index == 2 {
                                        if value == 7 { chars[0] = '7' };
                                        if value == 8 { chars[1] = '8' };
                                        if value == 9 { chars[2] = '9' };
                                    }
                                } else if value == 0 {
                                    chars[0] = '.';
                                    chars[1] = '.';
                                    chars[2] = '.';
                                    if row_index == 0 {
                                        if self.grid[row][column].is_value_valid(1) { chars[0] = '1' };
                                        if self.grid[row][column].is_value_valid(2) { chars[1] = '2' };
                                        if self.grid[row][column].is_value_valid(3) { chars[2] = '3' };
                                    } else if row_index == 1 {
                                        if self.grid[row][column].is_value_valid(4) { chars[0] = '4' };
                                        if self.grid[row][column].is_value_valid(5) { chars[1] = '5' };
                                        if self.grid[row][column].is_value_valid(6) { chars[2] = '6' };
                                    } else if row_index == 2 {
                                        if self.grid[row][column].is_value_valid(7) { chars[0] = '7' };
                                        if self.grid[row][column].is_value_valid(8) { chars[1] = '8' };
                                        if self.grid[row][column].is_value_valid(9) { chars[2] = '9' };
                                    }
                                } else {
                                    if row_index == 0 {
                                        if value == 1 { chars[0] = '1' };
                                        if value == 2 { chars[1] = '2' };
                                        if value == 3 { chars[2] = '3' };
                                    } else if row_index == 1 {
                                        if value == 4 { chars[0] = '4' };
                                        if value == 5 { chars[1] = '5' };
                                        if value == 6 { chars[2] = '6' };
                                    } else if row_index == 2 {
                                        if value == 7 { chars[0] = '7' };
                                        if value == 8 { chars[1] = '8' };
                                        if value == 9 { chars[2] = '9' };
                                    }
                                }
                            },
                        }
                        let print_string: String = chars.into_iter().collect();
                        print!("{}", print_string.bright_yellow());
                    }
                    println!();
                    if row_index == 2 {
                        println!("{}", "============+===========+===========".bright_cyan());
                    }
                }
            }
        } else {
            for (row, row_arr) in self.grid.iter().enumerate() {
                if row != 0 && row%BOX_SIDE == 0
                {
                    println!("{}", "=========+=========+=========".bright_cyan());
                }

                for (column, val) in row_arr.iter().enumerate() {
                    if column != 0 && column%BOX_SIDE == 0
                    {
                        print!("{} ", "|".bright_cyan());
                    } else {
                        print!(" ");
                    }
                    match val.get_value() {
                        Err(_) => print!("{} ", "X".bright_red()),
                        Ok(value) => {
                            if self.grid[row][column].is_set_by_user() {
                                print!("{} ", value.to_string().bright_green());
                            } else if value == 0 {
                                print!("{} ", ".".bright_yellow());
                            } else {
                                print!("{} ", value.to_string().bright_yellow());
                            }
                        },
                    }
                }
                println!();
            }
            println!();
        }
    }

    // Private utility functions
    // returns true if every cell is valid, false otherwise.
    fn is_valid(&self) -> bool {
        self.grid.iter().flat_map(|row| row.iter()).all(|cell| {
            cell.is_valid()
        })
    }

    // Get all the user settings
    fn get_user_settings(&self) -> Result<Vec<(usize, usize, u8)>, String> {
        let mut user_settings = Vec::new();
        for row in 0..GRID_SIZE {
            for column in 0..GRID_SIZE {
                if self.grid[row][column].is_set_by_user() {
                    match self.grid[row][column].get_value() {
                        Ok(value) if (1..=VALUE_MAX).contains(&value) => {
                            user_settings.push((row, column, value));
                        }
                        Ok(0) => {
                            return Err(format!(
                                "Unexpected unsolved user-set cell at row {} column {}",
                                row + 1,
                                column + 1
                            ));
                        }
                        Ok(value) => {
                            return Err(format!(
                                "Unexpected user-set value {} at row {} column {}",
                                value,
                                row + 1,
                                column + 1
                            ));
                        }
                        Err(()) => {
                            return Err(format!(
                                "Unexpected empty user-set cell at row {} column {}",
                                row + 1,
                                column + 1
                            ));
                        }
                    }
                }
            }
        }
        Ok(user_settings)
    }

    fn exact_cover_error_message(error: ExactCoverError) -> String {
        format!("Exact cover solver error: {error:?}")
    }

    fn exact_cover_clues(&self) -> Result<Vec<Assignment>, String> {
        let mut clues = Vec::with_capacity(GRID_SIZE * GRID_SIZE);

        for row in 0..GRID_SIZE {
            for column in 0..GRID_SIZE {
                if !self.grid[row][column].is_solved() {
                    continue;
                }

                let value = self.grid[row][column].get_value().map_err(|_| {
                    format!(
                        "Unexpected solved-cell error at row {} column {}",
                        row + 1,
                        column + 1
                    )
                })?;

                let clue = Assignment::new(row, column, value)
                    .map_err(Self::exact_cover_error_message)?;
                clues.push(clue);
            }
        }

        Ok(clues)
    }

    fn exact_cover_analyzer(&self) -> Result<ExactCoverAnalyzer, String> {
        let clues = self.exact_cover_clues()?;
        Ok(ExactCoverAnalyzer::new(&clues))
    }
}

// Private utility functions
// Calculates the dimensions of the 3x3 box that contains the input row and column
fn get_box_dimensions(row: usize, column: usize) -> (usize, usize, usize, usize) {
    let start_row = (row / BOX_SIDE) * BOX_SIDE;
    let end_row = start_row + BOX_SIDE;
    let start_column = (column / BOX_SIDE) * BOX_SIDE;
    let end_column = start_column + BOX_SIDE;
    (start_row, end_row, start_column, end_column)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_value_validates_row_column_and_value_bounds() {
        let mut grid = Grid::new();
        assert_eq!(grid.set_value(GRID_SIZE, 0, 1), Err("Invalid row 10".to_string()));
        assert_eq!(
            grid.set_value(0, GRID_SIZE, 1),
            Err("Invalid column 10".to_string())
        );
        assert_eq!(grid.set_value(0, 0, 0), Err("Invalid value 0".to_string()));
        assert_eq!(grid.set_value(0, 0, 10), Err("Invalid value 10".to_string()));
    }

    #[test]
    fn has_direct_conflict_checks_row_column_and_box() {
        let mut row_grid = Grid::new();
        assert_eq!(row_grid.set_value(0, 0, 5), Ok(true));
        assert!(row_grid.has_direct_conflict(0, 1, 5));
        assert!(!row_grid.has_direct_conflict(0, 1, 3));

        let mut column_grid = Grid::new();
        assert_eq!(column_grid.set_value(0, 0, 6), Ok(true));
        assert!(column_grid.has_direct_conflict(1, 0, 6));
        assert!(!column_grid.has_direct_conflict(1, 0, 2));

        let mut box_grid = Grid::new();
        assert_eq!(box_grid.set_value(0, 0, 7), Ok(true));
        assert!(box_grid.has_direct_conflict(1, 1, 7));
        assert!(!box_grid.has_direct_conflict(1, 1, 4));
    }

    #[test]
    fn user_delete_value_validates_row_column_bounds() {
        let mut grid = Grid::new();
        assert_eq!(
            grid.user_delete_value(GRID_SIZE, 0),
            Err("Invalid row 10".to_string())
        );
        assert_eq!(
            grid.user_delete_value(0, GRID_SIZE),
            Err("Invalid column 10".to_string())
        );
    }
}
