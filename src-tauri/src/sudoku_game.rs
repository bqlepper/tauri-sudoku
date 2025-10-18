use sudoku_grid::Grid;

pub mod sudoku_grid;

pub(super) struct Game {
    grid: Grid, // The grid holds 9x9 grid of cells
}

impl Game {
    pub(super) fn new() -> Game {
        Game {
            grid : Grid::new(),
        }
    }

    pub(super) fn get_grid(&mut self) -> String {
        self.grid.get_grid()
    }

    pub(super) fn set_debug(&mut self, on: bool)
    {
        self.grid.set_debug(on);
    }

    // Clear the whole grid of any user selections and restart the game
    pub(super) fn clear(&mut self) {
        self.grid.clear();
    }

    // Called to start a recursive brute force search
    pub(super) fn start_search(&mut self) -> Result<(), &str> {
        if self.grid.is_solved() {
            return Err("No need for search, the puzzle is already solved.");
        }
        println!("BQL TODO - This needs to be implemented in Searcher class in thread");
        Ok(())
    }

    // Called to manually stop the brute force solution search
    pub(super) fn stop_search(&mut self) {
        println!("BQL TODO - This needs to be implemented to stop Searcher class in thread");
    }

    // Clear a value that a user has previously set
    pub(super) fn user_delete_value(&mut self, row: usize, column: usize) {
        self.grid.user_delete_value(row, column);
    }

    // Attempts to set a value in the grid from values input by the user
    // row and column passed into this function are in the 0-8 range.  For debug output rows and columns are also in the 0-8 range.
    // However, for user entry and for displaying the rows and columns in the error messages, they are in the 1-9 range.
    pub(super) fn user_set_value(&mut self, row: usize, column: usize, value: u8) -> Result<(), String> {
        if row >= sudoku_grid::GRID_SIZE {
            return Err(format!("Invalid row {}", row+1));
        }
        if column >= sudoku_grid::GRID_SIZE {
            return Err(format!("Invalid column {}", column+1));
        }
        if value == 0 || value > 9 {
            return Err(format!("Invalid value {value}"));
        }

        // Create a trial grid to try out this setting
        let mut trial_grid = self.grid.clone();

        match trial_grid.set_value(row, column, value) {
            Err(out_msg) => return Err(out_msg),
            Ok(changed) => {
                if changed {
                    match trial_grid.run_all_checks() {
                        Err(_) => {
                            return Err(format!("{value} is not valid for row {} column {}!", row+1, column+1));
                        },
                        Ok(_) => {
                            trial_grid.run_extra_checks();
                            if trial_grid.is_solved() {
                                println!("Puzzle is solved!");
                            } else {
                                let mut solution_found = Vec::new();
                                trial_grid.solution_search(trial_grid.clone(), &mut solution_found, 1);
                                if solution_found.len() == 0 {
                                    return Err(format!("Setting {} at row {} column {} leads to no possible solutions!", value, row+1, column+1));
                                }
                            }
                            trial_grid.lock_set_by_user(row, column);
                            self.grid = trial_grid;
                        },
                    }
                }
            },
        }

        Ok(())
    }

    pub(super) fn print_grid(&self) {
        if self.grid.is_solved() {
            println!("\n**** Solved Puzzle ****");
        } else {
            println!("\n**** Current Puzzle ****");
        }
        self.grid.print_grid();
        if self.grid.is_solved() {
            println!("\nSOLVED!!!!!");
        }
    }
}
