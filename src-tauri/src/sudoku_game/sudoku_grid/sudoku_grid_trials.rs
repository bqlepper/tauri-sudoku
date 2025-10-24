use super::Grid;

const START_EXTRA_CHECKS: usize = 16;

impl Grid {
    // Try all remaining potential values and detect if they lead to an error situation (cell with no potential values)
    // If so, remove that potential value from the cell and run all the checks again.)
    fn run_try_checks (&mut self) -> bool {
        let mut result = false;
        // Find unsolved cells
        for row in 0..super::GRID_SIZE {
            for column in 0..super::GRID_SIZE {
                if self.grid[row][column].is_solved() {
                    continue;
                }
                for value in self.grid[row][column].get_value_list() {
                    let mut trial_grid = self.clone();
                    if let Ok(changed) = trial_grid.set_value(row, column, value) {
                        if changed {
                            if let Ok(_) = trial_grid.run_all_checks() {
                                continue; // This value is OK, move on to the next value
                            }
                        }
                    }

                    let trial_remove_result = trial_grid.grid[row][column].remove_value(value);
                    if trial_remove_result.is_ok() {
                        let trial_after_remove_result = trial_grid.run_all_checks();
                        if trial_after_remove_result.is_ok() {
                            self.replace(&trial_grid.grid);
                            result =   true;
                            break; // Need to restart the whole process since the grid has changed
                        }
                    }
                }
                if result { break }; // Need to restart the whole process since the grid has changed
            }
            if result { break }; // Need to restart the whole process since the grid has changed
        }
        result
    }

    fn get_solved_count(&self) -> usize {
        let mut solved_count = 0;
        for row in 0..super::GRID_SIZE {
            for column in 0..super::GRID_SIZE {
                if self.grid[row][column].is_solved() {
                    solved_count += 1;
                }
            }
        }
        solved_count
    }

    pub(crate) fn run_extra_checks(&mut self) {

        if self.get_solved_count() >= START_EXTRA_CHECKS {
            loop {
                // These try-and-check tests shouldn't be started until there are many potential values removed
                // 17 is supposedly the minimum number of clues for a valid Sudoku puzzle, so that is probably where
                // I should start, but I thought I would try a little smaller number.
                if !self.run_try_checks() {
                    break;
                }
            }
            assert!(self.is_valid(), "Invalid after running extra checks!");
        }
    }

    // Searches recursively for solutions.  If only 1 is found, the puzzle is solved.
    // Stops immedieately if more than 1 solution is found.
    pub(crate) fn run_final_check(&mut self) {

        if !self.is_solved() && self.get_solved_count() >= START_EXTRA_CHECKS {
            let mut solutions: Vec<Grid> = Vec::new();
            self.solution_search(self.clone(), &mut solutions, 2);
            assert!(solutions.len() > 0, "No solutions found after running extra checks!");
            if solutions.len() == 1 {
                self.replace(&solutions[0].grid);
            }
        }
    }

    pub(crate) fn is_solvable (&self) -> bool {
        if self.is_solved() || self.get_solved_count() < START_EXTRA_CHECKS {
            return true;
        }
        let mut solutions: Vec<Grid> = Vec::new();
        self.solution_search(self.clone(), &mut solutions, 1);
        solutions.len() > 0
    }

    pub(crate) fn count_solutions (&self) -> Result<usize, usize> {
        if self.is_solved() {
            println!("Already solved!!!!");
            self.print_grid();
            return Err(1);
        }
        let mut solutions: Vec<Grid> = Vec::new();
        self.solution_search(self.clone(), &mut solutions, 5);
        println!("Found {} solutions", solutions.len());
        if solutions.len() >= 5 {
            return Err(5);
        } else if solutions.len() == 0 {
            return Err(0);
        }
        Ok(solutions.len())
    }

    // Recursive method to try all remaining potential values and count the number of solutions that are still valid
    fn solution_search (&self,
                        in_grid: Grid,
                        solutions: &mut Vec<Grid>,
                        max_solutions: usize) {

        assert!(self.is_valid(), "Called solution_search with invalid puzzle!");
        assert!(!self.is_solved(), "Called solution_search with solvedd puzzle!");
        assert!(solutions.len() < max_solutions, "Called solution_search with already enough solutions!");

        // Find first non-solved cell
        let mut row: usize = 0;
        let mut column: usize = 0;
        'outer: for check_row in 0..super::GRID_SIZE {
            for check_column in 0..super::GRID_SIZE {
                if in_grid.grid[check_row][check_column].is_solved() {
                    continue;
                } else {
                    row = check_row;
                    column = check_column;
                    break 'outer;
                }
            }
        }

        for value in in_grid.grid[row][column].get_value_list() {
            let mut next_grid = in_grid.clone();
            match next_grid.set_value(row, column, value) {
                Err(_) => { continue; },
                Ok(changed) => {
                    assert!(changed, "Value {} in row {}, column {} was already set!", value, row, column);
                    match next_grid.run_all_checks() {
                        Err(_) => { continue;},
                        Ok(_) => {
                            next_grid.run_extra_checks();
                            if next_grid.is_solved() { // If we've found a possible solution
                                solutions.push(next_grid.clone());
                                return;
                            } else {
                                self.solution_search(next_grid, solutions, max_solutions);
                            }
                        },
                    }
                },
            }
            if solutions.len() >= max_solutions {
                return;
            }
        }
    }
}

