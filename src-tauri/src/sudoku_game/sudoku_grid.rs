use colored::*;
use sudoku_cell::Cell;

pub mod sudoku_cell;

pub(super) const GRID_SIZE: usize = 9;

// Calculates the dimensions of the 3x3 box that contains the input row and column
fn get_box_dimensions(row: usize, column: usize) -> (usize, usize, usize, usize) {
    let start_row = (row / 3) * 3;
    let end_row = start_row + 3;
    let start_column = (column / 3) * 3;
    let end_column = start_column + 3;
    (start_row, end_row, start_column, end_column)
}

fn get_new_grid() -> [[Cell; GRID_SIZE]; GRID_SIZE] {
    [
        [ Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new() ],
        [ Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new() ],
        [ Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new() ],
        [ Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new() ],
        [ Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new() ],
        [ Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new() ],
        [ Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new() ],
        [ Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new() ],
        [ Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new(), Cell::new() ],
    ]
}

// Grid has all the sudoku rule checking functions and prints out the grid to the command line
pub(super) struct Grid {
    grid: [[Cell; GRID_SIZE]; GRID_SIZE],
    debug_output: bool, // Allows the print out of all possible values of each cell to the screen for debug purposes
}

impl Grid {
    pub(super) fn new() -> Grid {
        Grid {
            grid : get_new_grid(),
            debug_output: false,
        }
    }

    pub(super) fn clear(&mut self) {
        self.grid.iter_mut().flat_map(|row| row.iter_mut()).for_each(|cell| {
            cell.clear();
        });
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

    fn replay_user_settings(&mut self, user_settings: Vec<(usize, usize, u8)>) {
        self.clear(); // Clear the grid
        for user_setting in user_settings {
            let result = self.set_value(user_setting.0, user_setting.1, user_setting.2);
            assert!(result.is_ok(), "Unexpected error resetting a user value! Row {}, Column {}, Value {}", user_setting.0, user_setting.1, user_setting.2);
            self.grid[user_setting.0][user_setting.1].lock_set_by_user();
        }

        let result = self.run_all_checks();
        assert!(result.is_ok(), "Unexpected error running checks due to resetting a user value!");
    }

    pub(super) fn clone(&self) -> Grid {
        let mut new_grid = Grid::new();
        if self.debug_output {
            new_grid.debug_output = true;
        }
        new_grid.replay_user_settings(self.get_user_settings());
        new_grid
    }

    pub(super) fn user_delete_value(&mut self, row: usize, column: usize) {
        // Don't let the user delete a value that they have not already set
        if self.grid[row][column].is_set_by_user() {
            self.grid[row][column].clear();
            // replay the remaining user settings on the cleared grid
            self.replay_user_settings(self.get_user_settings());
        }
    }

    // returns true if every cell is solved, false otherwise.
    pub(super) fn is_solved(&self) -> bool {
        self.grid.iter().flat_map(|row| row.iter()).all(|cell| {
            cell.is_solved()
        })
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
                assert!(partners.len() < p_count, "Too many partners found in row for row {row}, column {column}!");
                // If enough partners were found, remove those values from other cells in this row
                if partners.len() >= p_count-1 {
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
                assert!(partners.len() < p_count, "Too many partners found in column for  row {row}, column {column}!");
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
                assert!(partners.len() < p_count, "Too many partners found in box for  row {row}, column {column}!");
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

    // Try all remaining potential values and detect if they lead to an error situation (cell with no potential values)
    // If so, remove that potential value from the cell and run all the checks again.)
    fn run_try_checks (&mut self) -> bool {
        let mut result = false;
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
                            if let Ok(_) = trial_grid.run_all_checks() { continue; }
                        }
                    }

                    if self.debug_output {
                        println!("Found bad value {value} in row {row}, column {column}");
                    }

                    let remove_result = self.grid[row][column].remove_value(value);
                    assert!(remove_result.is_ok(), "Unexpected error removing value {value} from row {row}, column {column}!");
                    result = true;
                }
            }
        }
        result
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

            // These try-and-check tests shouldn't be started until there are many potential values removed
            // 17 is supposedly the minimum number of clues for a valid Sudoku puzzle, so that is probably where
            // I should start, but I thought I would try a little smaller number. 
            let mut any_try_checks_removed = false;
            if self.get_user_settings().len() >= 15 {
                any_try_checks_removed = self.run_try_checks();
                // BQL TODO am I always checking and blocking any set values that cause an error (no potential values in any cell)?
            }

            if !any_values_removed &&
               !any_new_sets &&
               !any_partners_removed &&
               !any_try_checks_removed
                { break; }
        }

        Ok(())
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
