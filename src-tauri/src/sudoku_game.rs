use sudoku_grid::Grid;

pub mod exact_cover;
pub mod sudoku_grid;
pub mod sudoku_constants;

pub struct Game {
    grid: Grid, // The grid holds 9x9 grid of cells
}

impl Game {
    pub fn new() -> Game {
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

    // Called to start count for good solutions
    pub(super) fn count_solutions(&mut self) -> Result<usize, usize> {
        self.grid.count_solutions()
    }

    // Clear a value that a user has previously set
    pub(super) fn user_delete_value(&mut self, row: usize, column: usize) -> Result<(), String> {
        self.grid.user_delete_value(row, column)
    }

    // Check if the puzzle is solved
    pub fn is_solved(&self) -> bool {
        self.grid.is_solved()
    }

    // Attempts to set a value in the grid from values input by the user
    // row and column passed into this function are in the 0-8 range.  For debug output rows and columns are also in the 0-8 range.
    // However, for user entry and for displaying the rows and columns in the error messages, they are in the 1-9 range.
    pub fn user_set_value(&mut self, row: usize, column: usize, value: u8) -> Result<(), String> {
        // Create a trial grid to try out this setting
        let mut trial_grid = self.grid.clone();

        match trial_grid.set_value(row, column, value) {
            Err(out_msg) => return Err(out_msg),
            Ok(changed) => {
                if changed {
                    if trial_grid.has_direct_conflict(row, column, value) {
                        return Err(format!("{value} is not valid for row {} column {}!", row+1, column+1));
                    }

                    trial_grid.run_exact_cover_checks();
                    trial_grid.lock_set_by_user(row, column);
                    if trial_grid.is_solved() {
                        println!("Puzzle is solved!");
                        self.grid = trial_grid;
                    } else if trial_grid.is_solvable() {
                        self.grid = trial_grid;
                        self.grid.print_remaining_solution_count_for_user_clues();
                    } else {
                        return Err(format!("Setting {} at row {} column {} leads to no possible solutions!", value, row+1, column+1));
                    }
                }
            },
        }

        Ok(())
    }

    pub fn print_grid(&self) {
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
