use super::Grid;

impl Grid {
    // Removes any solved cell values from all applicable rows, columns, and boxes
    fn remove_solved_values(&mut self) -> Result<bool, String> {
        let mut result = false;

        for row in 0..super::GRID_SIZE {
            for column in 0..super::GRID_SIZE {
                if self.grid[row][column].is_solved() {

                    let value = match self.grid[row][column].get_value() {
                        Err(_) => 0,
                        Ok(v) => v,
                    };

                    for column2 in 0..super::GRID_SIZE {
                        if column != column2 {
                            match self.grid[row][column2].remove_value(value) {
                                Err(()) => return Err(format!("Cannot remove {value} from row {row} column {column2}!")),
                                Ok(something_changed) => result |= something_changed,
                            }
                        }
                    }

                    for row2 in 0..super::GRID_SIZE {
                        if row != row2 {
                            match self.grid[row2][column].remove_value(value) {
                                Err(()) => return Err(format!("Cannot remove {value} from row {row2} column {column}!")),
                                Ok(something_changed) => result |= something_changed,
                            }
                        }
                    }

                    let bx = super::get_box_dimensions(row, column);
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

        for value in 1..=super::GRID_SIZE {
            // Check all rows for all values to see if only one cell in the row can be that value
            for row in 0..super::GRID_SIZE {
                value_count = 0;
                for column in 0..super::GRID_SIZE {
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
            for column2 in 0..super::GRID_SIZE {
                value_count = 0;
                for row2 in 0..super::GRID_SIZE {
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

            for row3 in (0..super::GRID_SIZE).step_by(3) {
                for column3 in (0..super::GRID_SIZE).step_by(3) {
                    let bx = super::get_box_dimensions(row3, column3);
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
        for row in 0..super::GRID_SIZE {
            for column in 0..super::GRID_SIZE {
                let p_count = self.grid[row][column].potential_value_count();
                assert!(p_count <= super::GRID_SIZE, "Cell at row {row}, column {column} has {p_count} potential values!");
                if self.grid[row][column].is_solved() || p_count == super::GRID_SIZE {
                    continue;
                }

                // Gather the other unsolved cells in the same row with the same potential values
                for column2 in 0..super::GRID_SIZE {
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
                for row2 in 0..super::GRID_SIZE {
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
                let sq = super::get_box_dimensions(row, column);
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

    pub(crate) fn run_all_checks(&mut self) -> Result<(), String> {

        loop {
            let mut any_values_removed = false;
            loop {
                match self.remove_solved_values() {
                    Err(out_msg) => return Err(out_msg),
                    Ok(something_removed) => if something_removed { any_values_removed = true; } else { break; },
                }
            }

            let mut any_new_sets = false;
            loop {
                match self.single_values() {
                    Err(out_msg) => return Err(out_msg),
                    Ok(something_set) => if something_set { any_new_sets = true; } else { break; },
                }
            }

            let mut any_partners_removed = false;
            loop {
                match self.run_partner_checks() {
                    Err(out_msg) => return Err(out_msg),
                    Ok(partners_removed) => if partners_removed { any_partners_removed = true; } else { break; },
                }
            }

            if !any_values_removed && !any_new_sets && !any_partners_removed
                { break; }
        }

        assert!(self.is_valid(), "Left run_all_checks with OK status, but invalid grid!");
        Ok(())
    }
}
