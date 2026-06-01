use super::Grid;

const MAX_SOLUTION_COUNT: usize = 50;
const ALL_VALUES_MASK: u16 = 0x1ff;
const START_EXTRA_CHECKS: usize = 10;
const START_SOLUTION_COUNT: usize = 16;

impl Grid {
    pub(crate) fn run_exact_cover_checks(&mut self) {
        if self.should_start_extra_checks() {
            if self.apply_exact_cover_checks().is_ok() {
                debug_assert!(self.is_valid(), "Invalid after running extra checks");
            }
            return;
        }

        if self.apply_simple_checks().is_ok() {
            debug_assert!(
                self.is_valid(),
                "Invalid after running direct candidate checks"
            );
        }
    }

    fn apply_simple_checks(&mut self) -> Result<(), String> {
        loop {
            let mut changed_any = false;

            for row in 0..super::GRID_SIZE {
                for column in 0..super::GRID_SIZE {
                    if self.grid[row][column].is_solved() {
                        continue;
                    }

                    let direct_mask = self.direct_constraint_mask_for_cell(row, column)?;
                    let current_mask = self.grid[row][column].candidate_mask();
                    let narrowed_mask = current_mask & direct_mask;

                    if narrowed_mask == 0 {
                        return Err(format!(
                            "No valid candidates remain at row {} column {}",
                            row + 1,
                            column + 1
                        ));
                    }

                    if self.grid[row][column]
                        .set_values(narrowed_mask)
                        .map_err(|_| {
                            format!(
                                "Invalid candidate mask at row {} column {}",
                                row + 1,
                                column + 1
                            )
                        })?
                    {
                        changed_any = true;
                    }
                }
            }

            if !changed_any {
                return Ok(());
            }
        }
    }

    fn direct_constraint_mask_for_cell(&self, row: usize, column: usize) -> Result<u16, String> {
        let mut mask = ALL_VALUES_MASK;

        for check_column in 0..super::GRID_SIZE {
            if check_column == column || !self.grid[row][check_column].is_solved() {
                continue;
            }
            let value = self.grid[row][check_column].get_value().map_err(|_| {
                format!(
                    "Unexpected solved-cell error at row {} column {}",
                    row + 1,
                    check_column + 1
                )
            })?;
            mask &= !value_to_bit(value);
        }

        for check_row in 0..super::GRID_SIZE {
            if check_row == row || !self.grid[check_row][column].is_solved() {
                continue;
            }
            let value = self.grid[check_row][column].get_value().map_err(|_| {
                format!(
                    "Unexpected solved-cell error at row {} column {}",
                    check_row + 1,
                    column + 1
                )
            })?;
            mask &= !value_to_bit(value);
        }

        let start_row = (row / super::BOX_SIDE) * super::BOX_SIDE;
        let end_row = start_row + super::BOX_SIDE;
        let start_column = (column / super::BOX_SIDE) * super::BOX_SIDE;
        let end_column = start_column + super::BOX_SIDE;
        for check_row in start_row..end_row {
            for check_column in start_column..end_column {
                if (check_row == row && check_column == column)
                    || !self.grid[check_row][check_column].is_solved()
                {
                    continue;
                }
                let value = self.grid[check_row][check_column].get_value().map_err(|_| {
                    format!(
                        "Unexpected solved-cell error at row {} column {}",
                        check_row + 1,
                        check_column + 1
                    )
                })?;
                mask &= !value_to_bit(value);
            }
        }

        Ok(mask)
    }

    // Returns true if at least one solution exists.
    pub(crate) fn is_solvable(&self) -> bool {
        if self.is_solved() {
            return true;
        }

        match self.count_exact_cover_solutions_with_cap(1) {
            Ok(count) => count > 0,
            Err(_) => false,
        }
    }

    // Counts solutions with a cap of 50 to preserve current command/UI contract.
    pub(crate) fn count_solutions(&self) -> Result<usize, usize> {
        if self.is_solved() {
            println!("Already solved");
            self.print_grid();
            return Err(1);
        }

        let solution_count = match self.count_exact_cover_solutions_with_cap(MAX_SOLUTION_COUNT) {
            Ok(count) => count,
            Err(_) => return Err(0),
        };

        println!("Found {} solutions", solution_count);
        if solution_count >= MAX_SOLUTION_COUNT {
            return Err(MAX_SOLUTION_COUNT);
        } else if solution_count == 0 {
            return Err(0);
        }
        Ok(solution_count)
    }

    pub(crate) fn print_remaining_solution_count_for_user_clues(&self) {
        let solution_count = self.solved_count();
        if solution_count < START_SOLUTION_COUNT {
            return;
        }

        match self.remaining_solution_count_for_user_clues() {
            Some(Ok(count)) => {
                if count >= MAX_SOLUTION_COUNT {
                    println!("There are at least {} solutions remaining", count);
                } else {
                    println!("There are exactly {} remaining solutions", count);
                }
            }
            Some(Err(error)) => {
                println!("Unable to count remaining solutions: {}", error);
            }
            None => {}
        }
    }

    // Private utility functions
    // Return true only if the user has entered enough clues
    // These extra checks are time consuming, so they should not be used until
    // the user has put in enough clues to perhaps make the puzzle solvable.
    // 17 is supposedly the minimum number of clues for a valid Sudoku puzzle with one solution.
    // Since exact-cover checks are fast now, start them earlier.
    fn solved_count(&self) -> usize {
        self.grid
            .iter()
            .flat_map(|row| row.iter())
            .filter(|cell| cell.is_solved())
            .count()
    }

    fn should_start_extra_checks(&self) -> bool {
        if self.solved_count() >= START_EXTRA_CHECKS {
            return true;
        }
        false
    }

    fn remaining_solution_count_for_user_clues(&self) -> Option<Result<usize, String>> {
        if self.solved_count() < START_SOLUTION_COUNT {
            return None;
        }

        Some(self.count_exact_cover_solutions_with_cap(MAX_SOLUTION_COUNT))
    }
}

fn value_to_bit(value: u8) -> u16 {
    1u16 << u32::from(value - 1)
}

#[cfg(test)]
mod tests {
    use super::{Grid, MAX_SOLUTION_COUNT, START_SOLUTION_COUNT};

    fn solved_grid_values() -> [[u8; 9]; 9] {
        [
            [5, 3, 4, 6, 7, 8, 9, 1, 2],
            [6, 7, 2, 1, 9, 5, 3, 4, 8],
            [1, 9, 8, 3, 4, 2, 5, 6, 7],
            [8, 5, 9, 7, 6, 1, 4, 2, 3],
            [4, 2, 6, 8, 5, 3, 7, 9, 1],
            [7, 1, 3, 9, 2, 4, 8, 5, 6],
            [9, 6, 1, 5, 3, 7, 2, 8, 4],
            [2, 8, 7, 4, 1, 9, 6, 3, 5],
            [3, 4, 5, 2, 8, 6, 1, 7, 9],
        ]
    }

    fn build_grid_with_omitted_digits(first_omitted: u8, second_omitted: u8) -> Grid {
        let solved = solved_grid_values();
        let mut grid = Grid::new();

        for (row, row_values) in solved.iter().enumerate() {
            for (column, &value) in row_values.iter().enumerate() {
                if value == first_omitted || value == second_omitted {
                    continue;
                }
                assert_eq!(grid.set_value(row, column, value), Ok(true));
            }
        }

        grid
    }

    fn build_user_clue_grid_with_omitted_digits(first_omitted: u8, second_omitted: u8) -> Grid {
        let solved = solved_grid_values();
        let mut grid = Grid::new();

        for (row, row_values) in solved.iter().enumerate() {
            for (column, &value) in row_values.iter().enumerate() {
                if value == first_omitted || value == second_omitted {
                    continue;
                }
                assert_eq!(grid.set_value(row, column, value), Ok(true));
                grid.lock_set_by_user(row, column);
            }
        }

        grid
    }

    fn build_grid_with_user_clues(clue_count: usize) -> Grid {
        let solved = solved_grid_values();
        let mut grid = Grid::new();
        let mut set_count = 0usize;

        for (row, row_values) in solved.iter().enumerate() {
            for (column, &value) in row_values.iter().enumerate() {
                if set_count >= clue_count {
                    return grid;
                }
                assert_eq!(grid.set_value(row, column, value), Ok(true));
                grid.lock_set_by_user(row, column);
                set_count += 1;
            }
        }

        grid
    }

    fn assert_has_multiple_solutions_below_cap(first_omitted: u8, second_omitted: u8) {
        let grid = build_grid_with_omitted_digits(first_omitted, second_omitted);
        let count = grid
            .count_exact_cover_solutions_with_cap(MAX_SOLUTION_COUNT)
            .expect("counting should succeed for a valid partially-clued puzzle");

        assert!(
            (2..MAX_SOLUTION_COUNT).contains(&count),
            "expected 2..{} solutions when omitting digits {} and {}, got {}",
            MAX_SOLUTION_COUNT,
            first_omitted,
            second_omitted,
            count
        );
    }

    #[test]
    fn detects_multiple_solutions_below_cap_when_omitting_1_and_2() {
        assert_has_multiple_solutions_below_cap(1, 2);
    }

    #[test]
    fn detects_multiple_solutions_below_cap_when_omitting_3_and_4() {
        assert_has_multiple_solutions_below_cap(3, 4);
    }

    #[test]
    fn detects_multiple_solutions_below_cap_when_omitting_5_and_6() {
        assert_has_multiple_solutions_below_cap(5, 6);
    }

    #[test]
    fn detects_multiple_solutions_below_cap_when_omitting_8_and_9() {
        assert_has_multiple_solutions_below_cap(8, 9);
    }

    #[test]
    fn exact_remaining_solution_count_matches_known_high_clue_case() {
        let grid = build_user_clue_grid_with_omitted_digits(1, 3);

        assert_eq!(grid.solved_count(), 63);
        assert!(grid.solved_count() > START_SOLUTION_COUNT);
        assert_eq!(grid.remaining_solution_count_for_user_clues(), Some(Ok(4)));
    }

    #[test]
    fn direct_checks_solve_single_missing_value_in_box_before_extra_check_threshold() {
        let mut grid = Grid::new();

        assert_eq!(grid.set_value(0, 0, 1), Ok(true));
        assert_eq!(grid.set_value(0, 1, 2), Ok(true));
        assert_eq!(grid.set_value(0, 2, 3), Ok(true));
        assert_eq!(grid.set_value(1, 0, 6), Ok(true));
        assert_eq!(grid.set_value(1, 1, 5), Ok(true));
        assert_eq!(grid.set_value(1, 2, 4), Ok(true));
        assert_eq!(grid.set_value(2, 0, 7), Ok(true));
        assert_eq!(grid.set_value(2, 2, 8), Ok(true));

        grid.run_exact_cover_checks();

        assert_eq!(grid.grid[2][1].get_value(), Ok(9));
        assert!(!grid.grid[2][1].is_set_by_user());
    }

    #[test]
    fn exact_remaining_solution_count_is_not_attempted_below_user_clue_threshold() {
        let grid = build_grid_with_user_clues(START_SOLUTION_COUNT - 1);
        assert!(grid.remaining_solution_count_for_user_clues().is_none());
    }

    #[test]
    fn exact_remaining_solution_count_runs_after_user_clue_threshold() {
        let grid = build_grid_with_user_clues(80);
        assert_eq!(
            grid.remaining_solution_count_for_user_clues(),
            Some(Ok(1))
        );
    }
}
