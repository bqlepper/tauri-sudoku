use sudoku_grid::Grid;

pub mod sudoku_grid;

pub(super) struct Game {
    grid: Grid, // The grid holds 9x9 grid of cells
}

impl Game {
    pub(super) fn new() -> Game {
        Game { grid: Grid::new() }
    }

    pub(super) fn get_grid(&mut self) -> String {
        self.grid.get_grid()
    }

    pub(super) fn set_debug(&mut self, on: bool) {
        self.grid.set_debug(on);
    }

    // Clear the whole grid of any user selections and restart the game
    pub(super) fn clear(&mut self) {
        self.grid.clear();
    }

    // Called to start count for good solutions
    pub(super) fn count_solutions(&mut self) -> Result<usize, usize> {
        self.grid.count_solutions()
    }

    // Clear a value that a user has previously set
    pub(super) fn user_delete_value(&mut self, row: usize, column: usize) {
        self.grid.user_delete_value(row, column);
    }

    // Attempts to set a value in the grid from values input by the user
    // row and column passed into this function are in the 0-8 range.  For debug output rows and columns are also in the 0-8 range.
    // However, for user entry and for displaying the rows and columns in the error messages, they are in the 1-9 range.
    pub(super) fn user_set_value(
        &mut self,
        row: usize,
        column: usize,
        value: u8,
    ) -> Result<(), String> {
        // Create a trial grid to try out this setting
        let mut trial_grid = self.grid.clone();

        match trial_grid.set_value(row, column, value) {
            Err(out_msg) => return Err(out_msg),
            Ok(changed) => {
                if changed {
                    match trial_grid.run_all_checks() {
                        Err(_) => {
                            return Err(format!(
                                "{value} is not valid for row {} column {}!",
                                row + 1,
                                column + 1
                            ));
                        }
                        Ok(_) => {
                            trial_grid.run_extra_checks();
                            trial_grid.run_final_check();
                            if trial_grid.is_solved() {
                                println!("Puzzle is solved!");
                                self.grid = trial_grid;
                            } else if trial_grid.is_solvable() {
                                trial_grid.lock_set_by_user(row, column);
                                self.grid = trial_grid;
                            } else {
                                return Err(format!("Setting {} at row {} column {} leads to no possible solutions!", value, row+1, column+1));
                            }
                        }
                    }
                }
            }
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
