use colored::*;
use sudoku_cell::Cell;

pub mod sudoku_cell;

pub(super) const GRID_SIZE: usize = 9;
const START_EXTRA_CHECKS: usize = 16;

// Calculates the dimensions of the 3x3 box that contains the input row and column
fn get_box_dimensions(row: usize, column: usize) -> (usize, usize, usize, usize) {
    let start_row = (row / 3) * 3;
    let end_row = start_row + 3;
    let start_column = (column / 3) * 3;
    let end_column = start_column + 3;
    (start_row, end_row, start_column, end_column)
}

// Grid has all the sudoku rule checking functions and prints out the grid to the command line
pub(super) struct Grid {
    grid: [[Cell; GRID_SIZE]; GRID_SIZE],
    debug_output: bool, // Allows the print out of all possible values of each cell to the screen for debug purposes
}

impl Grid {
    pub(super) fn new() -> Grid {
        let new_grid = [
            [ Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new() ],
            [ Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new() ],
            [ Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new() ],
            [ Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new() ],
            [ Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new() ],
            [ Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new() ],
            [ Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new() ],
            [ Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new() ],
            [ Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new() ],
        ];

        Grid {
            grid : new_grid,
            debug_output: false,
        }
    }

    pub(super) fn clone(&self) -> Grid {
        let new_grid = [
            [ self.grid[0][0].clone(), self.grid[0][1].clone(), self.grid[0][2].clone(), self.grid[0][3].clone(), self.grid[0][4].clone(), self.grid[0][5].clone(), self.grid[0][6].clone(), self.grid[0][7].clone(), self.grid[0][8].clone() ],
            [ self.grid[1][0].clone(), self.grid[1][1].clone(), self.grid[1][2].clone(), self.grid[1][3].clone(), self.grid[1][4].clone(), self.grid[1][5].clone(), self.grid[1][6].clone(), self.grid[1][7].clone(), self.grid[1][8].clone() ],
            [ self.grid[2][0].clone(), self.grid[2][1].clone(), self.grid[2][2].clone(), self.grid[2][3].clone(), self.grid[2][4].clone(), self.grid[2][5].clone(), self.grid[2][6].clone(), self.grid[2][7].clone(), self.grid[2][8].clone() ],
            [ self.grid[3][0].clone(), self.grid[3][1].clone(), self.grid[3][2].clone(), self.grid[3][3].clone(), self.grid[3][4].clone(), self.grid[3][5].clone(), self.grid[3][6].clone(), self.grid[3][7].clone(), self.grid[3][8].clone() ],
            [ self.grid[4][0].clone(), self.grid[4][1].clone(), self.grid[4][2].clone(), self.grid[4][3].clone(), self.grid[4][4].clone(), self.grid[4][5].clone(), self.grid[4][6].clone(), self.grid[4][7].clone(), self.grid[4][8].clone() ],
            [ self.grid[5][0].clone(), self.grid[5][1].clone(), self.grid[5][2].clone(), self.grid[5][3].clone(), self.grid[5][4].clone(), self.grid[5][5].clone(), self.grid[5][6].clone(), self.grid[5][7].clone(), self.grid[5][8].clone() ],
            [ self.grid[6][0].clone(), self.grid[6][1].clone(), self.grid[6][2].clone(), self.grid[6][3].clone(), self.grid[6][4].clone(), self.grid[6][5].clone(), self.grid[6][6].clone(), self.grid[6][7].clone(), self.grid[6][8].clone() ],
            [ self.grid[7][0].clone(), self.grid[7][1].clone(), self.grid[7][2].clone(), self.grid[7][3].clone(), self.grid[7][4].clone(), self.grid[7][5].clone(), self.grid[7][6].clone(), self.grid[7][7].clone(), self.grid[7][8].clone() ],
            [ self.grid[8][0].clone(), self.grid[8][1].clone(), self.grid[8][2].clone(), self.grid[8][3].clone(), self.grid[8][4].clone(), self.grid[8][5].clone(), self.grid[8][6].clone(), self.grid[8][7].clone(), self.grid[8][8].clone() ]
        ];

        Grid {
            grid: new_grid,
            debug_output: self.debug_output,
        }
    }

    fn replace(&mut self, grid_in: &[[Cell; 9]; 9]) {
        for row in 0..GRID_SIZE {
            for column in 0..GRID_SIZE {
                self.grid[row][column] = grid_in[row][column].clone();
            }
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

    // returns true if every cell is valid, false otherwise.
    pub(super) fn is_valid(&self) -> bool {
        self.grid.iter().flat_map(|row| row.iter()).all(|cell| {
            cell.is_valid()
        })
    }

    pub(super) fn set_debug(&mut self, on: bool) { self.debug_output = on; }

    pub(super) fn lock_set_by_user(&mut self, row: usize, column:usize) { self.grid[row][column].lock_set_by_user(); }

    // stringify the whole grid
    pub(super) fn get_grid(&self) -> String {
        let mut grid_string = String::new();
        for row in 0..GRID_SIZE {
            for column in 0..GRID_SIZE {
                if self.grid[row][column].is_set_by_user() {
                    grid_string.push('u');
                } else {
                    grid_string.push('-');
                }
                grid_string.push(match self.grid[row][column].get_value() {
                    Err(_) => 'x',
                    Ok(v) => (v + b'0') as char,
                });
            }
        }
        grid_string
    }

    // Get all the user settings
    fn get_user_settings(&self) -> Vec<(usize, usize, u8)> {
        let mut user_settings = Vec::new();
        for row in 0..GRID_SIZE {
            for column in 0..GRID_SIZE {
                if self.grid[row][column].is_set_by_user() {
                    let result = self.grid[row][column].get_value();
                    assert!(result.is_ok(), "Unexpected error no value for a user set cell!");
                    user_settings.push((row, column, result.unwrap()));
                }
            }
        }
        user_settings
    }

    pub(super) fn user_delete_value(&mut self, row: usize, column: usize) {
        // Don't let the user delete a value that they have not already set
        if self.grid[row][column].is_set_by_user() {
            self.grid[row][column].clear();  //Clear the cells user settings
            let replay_user_settings = self.get_user_settings(); // Save all the other user settings
            self.clear(); // Clear the whole grid
            for user_setting in replay_user_settings { // Replay all the other user settings
                let result = self.set_value(user_setting.0, user_setting.1, user_setting.2);
                assert!(result.is_ok(), "Unexpected error resetting a user value! Row {}, Column {}, Value {}", user_setting.0, user_setting.1, user_setting.2);
                self.grid[user_setting.0][user_setting.1].lock_set_by_user();
            }

            let result = self.run_all_checks();
            assert!(result.is_ok(), "Unexpected error running checks due to resetting a user value!");
            self.run_extra_checks();
        }
    }

    // Removes any solved cell values from all applicable rows, columns, and boxes
    fn remove_solved_values(&mut self) -> Result<bool, String> {
        let mut result = false;

        for row in 0..GRID_SIZE {
            for column in 0..GRID_SIZE {
                if self.grid[row][column].is_solved() {

                    let value = match self.grid[row][column].get_value() {
                        Err(_) => 0,
                        Ok(v) => v,
                    };

                    for column2 in 0..GRID_SIZE {
                        if column != column2 {
                            match self.grid[row][column2].remove_value(value) {
                                Err(()) => return Err(format!("Cannot remove {value} from row {row} column {column2}!")),
                                Ok(something_changed) => result |= something_changed,
                            }
                        }
                    }

                    for row2 in 0..GRID_SIZE {
                        if row != row2 {
                            match self.grid[row2][column].remove_value(value) {
                                Err(()) => return Err(format!("Cannot remove {value} from row {row2} column {column}!")),
                                Ok(something_changed) => result |= something_changed,
                            }
                        }
                    }

                    let bx = get_box_dimensions(row, column);
                    for row3 in bx.0..bx.1 {
                        for column3 in bx.2..bx.3 {
                            if row != row3 || column != column3 {
                                match self.grid[row3][column3].remove_value(value) {
                                    Err(()) => return Err(format!("Cannot remove {value} from row {row3} column {column3}!")),
                                    Ok(something_changed) => result |= something_changed,
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(result)
    }

    // Looks for rows, columns, and boxes that have only 1 cell that can be a particular value
    // BQL TODO I need to refactor this function.  I use a lot of local variables and  a lot of similar code repeated.
    fn single_values(&mut self) -> Result<bool, String> {
        let mut result = false;
        let mut value_count: usize;
        let mut value_row: usize = 0;
        let mut value_column: usize = 0;

        for value in 1..=GRID_SIZE {
            // Check all rows for all values to see if only one cell in the row can be that value
            for row in 0..GRID_SIZE {
                value_count = 0;
                for column in 0..GRID_SIZE {
                    if self.grid[row][column].is_value_valid(value as u8) {
                        value_count += 1;
                        value_row = row;
                        value_column = column;
                    };
                    if value_count > 1 {
                        break;
                    }
                }
                if value_count < 1 {
                    return Err(format!("Unexpected error row {row} has no value {value}!"));
                }
                if value_count == 1 {
                    match self.grid[value_row][value_column].set_value(value as u8) {
                        Err(()) => return Err(format!("Unexpected row error: row {value_row} col {value_column} val {value}!")),
                        Ok(something_changed) => result |= something_changed,
                    }
                }
            }

            // Check all columns for all values to see if only one cell in the column can be that value
            for column2 in 0..GRID_SIZE {
                value_count = 0;
                for row2 in 0..GRID_SIZE {
                    if self.grid[row2][column2].is_value_valid(value as u8) {
                        value_count += 1;
                        value_row = row2;
                        value_column = column2;
                    };
                    if value_count > 1 {
                        break;
                    }
                }
                if value_count < 1 {
                    return Err(format!("Unexpected error column {column2} has no value {value}!"));
                }
                if value_count == 1 {
                    match self.grid[value_row][value_column].set_value(value as u8) {
                        Err(()) => return Err(format!("Unexpected column error: row {value_row} col {value_column} val {value}!")),
                        Ok(something_changed2) => result |= something_changed2,
                    }
                }
            }

            for row3 in (0..GRID_SIZE).step_by(3) {
                for column3 in (0..GRID_SIZE).step_by(3) {
                    let bx = get_box_dimensions(row3, column3);
                    value_count = 0;
                    for row3 in bx.0..bx.1 {
                        for column3 in bx.2..bx.3 {
                            if self.grid[row3][column3].is_value_valid(value as u8) {
                                value_count += 1;
                                value_row = row3;
                                value_column = column3;
                            };
                            if value_count > 1 {
                                break;
                            }
                        }
                        if value_count > 1 {
                            break;
                        }
                    }
                    if value_count < 1 {
                        return Err(format!("Unexpected error: cell for row {row3} column {column3} has no value {value}!"));
                    }
                    if value_count == 1 {
                        match self.grid[value_row][value_column].set_value(value as u8) {
                            Err(()) => return Err(format!("Unexpected box error: row {value_row} col {value_column} val {value}!")),
                            Ok(something_changed3) => result |= something_changed3,
                        }
                    }
                }
            }
        }
        Ok(result)
    }

    fn remove_partners(&mut self, p_values: u16, non_partners: &Vec<(usize, usize)>) -> Result<bool, String> {
        let mut result = false;
        for np in non_partners {
            match self.grid[np.0][np.1].remove_values(p_values) {
                Ok(something_changed) => result |= something_changed,
                Err(()) => return Err(format!("Removing partners from row {}, column {}, values {}!", np.0, np.1, p_values)),
            }
        }

        Ok(result)
    }

    // Check in all rows, columns, and boxes for matching potential partner values that
    // prohibit those values from being used in other cells in the same row, column, or box.
    // For example if two cells in the same row both have only 1 and 2 as potential values,
    // then neither 1 or 2 can be values in any other cells in that row.
    fn run_partner_checks(&mut self) -> Result<bool, String> {
        let mut partners: Vec<(usize, usize)> = Vec::new(); // potential partners
        let mut non_partners: Vec<(usize, usize)> = Vec::new(); // potential partners
        let mut result = false;
        // For each unsolved cell
        for row in 0..GRID_SIZE {
            for column in 0..GRID_SIZE {
                let p_count = self.grid[row][column].potential_value_count();
                assert!(p_count <= GRID_SIZE, "Cell at row {row}, column {column} has {p_count} potential values!");
                if self.grid[row][column].is_solved() || p_count == GRID_SIZE {
                    continue;
                }

                // Gather the other unsolved cells in the same row with the same potential values
                for column2 in 0..GRID_SIZE {
                    if column == column2 || self.grid[row][column2].is_solved() {
                        continue;
                    }
                    if self.grid[row][column2].is_partner(self.grid[row][column].get_values()) {
                        partners.push((row, column2));
                    } else {
                        non_partners.push((row, column2));
                    }
                }

                // Shouldn't be able to have more partners than potential values
                if partners.len() >= p_count {
                    return Err(format!("Too many partners found in row for row {row}, column, {column}"));
                }

                // If enough partners were found, remove those values from other cells in this row
                if partners.len() == p_count-1 {
                    match self.remove_partners(self.grid[row][column].get_values(), &non_partners) {
                        Ok(something_changed) => result |= something_changed,
                        Err(error_out) => return Err(format!("Error partner rows: row {row}, column, {column}: {error_out}")),
                    };
                }

                partners.clear();
                non_partners.clear();

                // Gather the other unsolved cells in the same column with the same potential values
                for row2 in 0..GRID_SIZE {
                    if row == row2 || self.grid[row2][column].is_solved() {
                        continue;
                    }
                    if self.grid[row2][column].is_partner(self.grid[row][column].get_values()) {
                        partners.push((row2, column));
                    } else {
                        non_partners.push((row2, column));
                    }
                }

                // Shouldn't be able to have more partners than potential values
                if partners.len() >= p_count {
                    return Err(format!("Too many partners found in column for row {row}, column, {column}"));
                }

                // If enough partners were found, remove those values from the other cells in this column
                if partners.len() == p_count-1 {
                    match self.remove_partners(self.grid[row][column].get_values(), &non_partners) {
                        Ok(something_changed) => result |= something_changed,
                        Err(error_out) => return Err(format!("Error partner columns: row {row}, column, {column}: {error_out}")),
                    };
                }

                partners.clear();
                non_partners.clear();

                // Gather the other unsolved cells in the same box with the same potential values
                let sq = get_box_dimensions(row, column);
                for row3 in sq.0..sq.1 {
                    for column3 in sq.2..sq.3 {
                        if (row == row3 && column == column3) || self.grid[row3][column3].is_solved() {
                            continue;
                        }
                        if self.grid[row3][column3].is_partner(self.grid[row][column].get_values()) {
                            partners.push((row3, column3));
                        } else {
                            non_partners.push((row3, column3));
                        }
                    }
                }

                // Shouldn't be able to have more partners than potential values
                if partners.len() >= p_count {
                    return Err(format!("Too many partners found in box for row {row}, column, {column}"));
                }

                // If enough partners were found, remove those values from the other cells in this box
                if partners.len() == p_count-1 {
                    match self.remove_partners(self.grid[row][column].get_values(), &non_partners) {
                        Ok(something_changed) => result |= something_changed,
                        Err(error_out) => return Err(format!("Error partner cells: row {row}, column, {column}: {error_out}")),
                    };
                }

                partners.clear();
                non_partners.clear();
            }
        }

        Ok(result)
    }

    pub(super) fn run_all_checks(&mut self) -> Result<(), String> {

        if self.debug_output {
            println!("\n**** Starting checks loop ****");
            self.print_grid();
        }

        loop {
            let mut any_values_removed = false;
            loop {
                match self.remove_solved_values() {
                    Err(out_msg) => return Err(out_msg),
                    Ok(something_removed) => if something_removed { any_values_removed = true; } else { break; },
                }
            }

            if self.debug_output {
                if any_values_removed {
                    println!("\n**** After removing values ****");
                } else {
                    println!("\n**** No values removed ****");
                }
                self.print_grid();
            }

            let mut any_new_sets = false;
            loop {
                match self.single_values() {
                    Err(out_msg) => return Err(out_msg),
                    Ok(something_set) => if something_set { any_new_sets = true; } else { break; },
                }
            }

            if self.debug_output {
                if any_new_sets {
                    println!("\n**** After setting single values ****");
                } else {
                    println!("\n**** No values set ****");
                }
                self.print_grid();
            }

            let mut any_partners_removed = false;
            loop {
                match self.run_partner_checks() {
                    Err(out_msg) => return Err(out_msg),
                    Ok(partners_removed) => if partners_removed { any_partners_removed = true; } else { break; },
                }
            }

            if self.debug_output {
                println!("\n**** After partner checking ****");
                self.print_grid();
            }

            if !any_values_removed && !any_new_sets && !any_partners_removed
                { break; }
        }

        assert!(self.is_valid(), "Left run_all_checks with OK status, but invalid grid!");
        Ok(())
    }

    // Try all remaining potential values and detect if they lead to an error situation (cell with no potential values)
    // If so, remove that potential value from the cell and run all the checks again.)
    fn run_try_checks (&mut self) -> bool {
        let mut result = false;
        if self.debug_output {
            println!("Looking for bad values");
        }
        // Find unsolved cells
        for row in 0..GRID_SIZE {
            for column in 0..GRID_SIZE {
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

                    if self.debug_output {
                        println!("Found a potential bad value {value} in row {row}, column {column}");
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
        if self.debug_output {
            if result {
                println!("Removed a bad value.");
            } else {
                println!("Did not find bad values");
            }
        }
        result
    }

    fn get_solved_count(&self) -> usize {
        let mut solved_count = 0;
        for row in 0..GRID_SIZE {
            for column in 0..GRID_SIZE {
                if self.grid[row][column].is_solved() {
                    solved_count += 1;
                }
            }
        }
        solved_count
    }

    pub(super) fn run_extra_checks(&mut self) {

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
    pub(super) fn run_final_check(&mut self) {

        if !self.is_solved() && self.get_solved_count() >= START_EXTRA_CHECKS {
            let mut solutions: Vec<Grid> = Vec::new();
            self.solution_search(self.clone(), &mut solutions, 2);
            assert!(solutions.len() > 0, "No solutions found after running extra checks!");
            if solutions.len() == 1 {
                println!("Unique solution found in final check!");
                self.replace(&solutions[0].grid);
            }
        }
    }

    pub(super) fn is_solvable (&self) -> bool {
        if self.is_solved() || self.get_solved_count() < START_EXTRA_CHECKS {
            return true;
        }
        let mut solutions: Vec<Grid> = Vec::new();
        self.solution_search(self.clone(), &mut solutions, 1);
        solutions.len() > 0
    }

    pub(super) fn count_solutions (&self) -> Result<usize, usize> {
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
        'outer: for check_row in 0..GRID_SIZE {
            for check_column in 0..GRID_SIZE {
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

    // Returns false if this value was already set, otherwise returns true to indicate a change
    pub(super) fn set_value(&mut self, row: usize, column: usize, value: u8) -> Result<bool, String> {
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
                if row != 0 && row%3 == 0
                {
                    println!("{}", "------------+-----------+-----------".bright_cyan());
                }

                for row_index in 0..3 {
                    for (column, val) in row_arr.iter().enumerate() {
                        if column != 0 && column%3 == 0
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
                                    chars[0] = '-';
                                    chars[1] = '-';
                                    chars[2] = '-';
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
                        println!("{}", "------------+-----------+-----------".bright_cyan());
                    }
                }
            }
        } else {
            for (row, row_arr) in self.grid.iter().enumerate() {
                if row != 0 && row%3 == 0
                {
                    println!("{}", "---------+---------+---------".bright_cyan());
                }

                for (column, val) in row_arr.iter().enumerate() {
                    if column != 0 && column%3 == 0
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
                                print!("{} ", "-".bright_yellow());
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
}

