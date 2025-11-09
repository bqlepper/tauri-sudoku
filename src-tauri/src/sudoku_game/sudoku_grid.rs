use colored::*;
use sudoku_cell::Cell;

pub mod sudoku_cell;
pub mod sudoku_grid_checks;
pub mod sudoku_grid_trials;

const GRID_SIZE: usize = 9;

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
            [
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
            ],
            [
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
            ],
            [
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
            ],
            [
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
            ],
            [
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
            ],
            [
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
            ],
            [
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
            ],
            [
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
            ],
            [
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
            ],
        ];

        Grid {
            grid: new_grid,
            debug_output: false,
        }
    }

    pub(super) fn clone(&self) -> Grid {
        let new_grid = [
            [
                self.grid[0][0].clone(),
                self.grid[0][1].clone(),
                self.grid[0][2].clone(),
                self.grid[0][3].clone(),
                self.grid[0][4].clone(),
                self.grid[0][5].clone(),
                self.grid[0][6].clone(),
                self.grid[0][7].clone(),
                self.grid[0][8].clone(),
            ],
            [
                self.grid[1][0].clone(),
                self.grid[1][1].clone(),
                self.grid[1][2].clone(),
                self.grid[1][3].clone(),
                self.grid[1][4].clone(),
                self.grid[1][5].clone(),
                self.grid[1][6].clone(),
                self.grid[1][7].clone(),
                self.grid[1][8].clone(),
            ],
            [
                self.grid[2][0].clone(),
                self.grid[2][1].clone(),
                self.grid[2][2].clone(),
                self.grid[2][3].clone(),
                self.grid[2][4].clone(),
                self.grid[2][5].clone(),
                self.grid[2][6].clone(),
                self.grid[2][7].clone(),
                self.grid[2][8].clone(),
            ],
            [
                self.grid[3][0].clone(),
                self.grid[3][1].clone(),
                self.grid[3][2].clone(),
                self.grid[3][3].clone(),
                self.grid[3][4].clone(),
                self.grid[3][5].clone(),
                self.grid[3][6].clone(),
                self.grid[3][7].clone(),
                self.grid[3][8].clone(),
            ],
            [
                self.grid[4][0].clone(),
                self.grid[4][1].clone(),
                self.grid[4][2].clone(),
                self.grid[4][3].clone(),
                self.grid[4][4].clone(),
                self.grid[4][5].clone(),
                self.grid[4][6].clone(),
                self.grid[4][7].clone(),
                self.grid[4][8].clone(),
            ],
            [
                self.grid[5][0].clone(),
                self.grid[5][1].clone(),
                self.grid[5][2].clone(),
                self.grid[5][3].clone(),
                self.grid[5][4].clone(),
                self.grid[5][5].clone(),
                self.grid[5][6].clone(),
                self.grid[5][7].clone(),
                self.grid[5][8].clone(),
            ],
            [
                self.grid[6][0].clone(),
                self.grid[6][1].clone(),
                self.grid[6][2].clone(),
                self.grid[6][3].clone(),
                self.grid[6][4].clone(),
                self.grid[6][5].clone(),
                self.grid[6][6].clone(),
                self.grid[6][7].clone(),
                self.grid[6][8].clone(),
            ],
            [
                self.grid[7][0].clone(),
                self.grid[7][1].clone(),
                self.grid[7][2].clone(),
                self.grid[7][3].clone(),
                self.grid[7][4].clone(),
                self.grid[7][5].clone(),
                self.grid[7][6].clone(),
                self.grid[7][7].clone(),
                self.grid[7][8].clone(),
            ],
            [
                self.grid[8][0].clone(),
                self.grid[8][1].clone(),
                self.grid[8][2].clone(),
                self.grid[8][3].clone(),
                self.grid[8][4].clone(),
                self.grid[8][5].clone(),
                self.grid[8][6].clone(),
                self.grid[8][7].clone(),
                self.grid[8][8].clone(),
            ],
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
        self.grid
            .iter_mut()
            .flat_map(|row| row.iter_mut())
            .for_each(|cell| {
                cell.clear();
            });
    }

    // returns true if every cell is solved, false otherwise.
    pub(super) fn is_solved(&self) -> bool {
        self.grid
            .iter()
            .flat_map(|row| row.iter())
            .all(|cell| cell.is_solved())
    }

    // returns true if every cell is valid, false otherwise.
    fn is_valid(&self) -> bool {
        self.grid
            .iter()
            .flat_map(|row| row.iter())
            .all(|cell| cell.is_valid())
    }

    pub(super) fn set_debug(&mut self, on: bool) {
        self.debug_output = on;
    }

    pub(super) fn lock_set_by_user(&mut self, row: usize, column: usize) {
        self.grid[row][column].lock_set_by_user();
    }

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
                    assert!(
                        result.is_ok(),
                        "Unexpected error no value for a user set cell!"
                    );
                    user_settings.push((row, column, result.unwrap()));
                }
            }
        }
        user_settings
    }

    pub(super) fn user_delete_value(&mut self, row: usize, column: usize) {
        // Don't let the user delete a value that they have not already set
        if self.grid[row][column].is_set_by_user() {
            self.grid[row][column].clear(); //Clear the cells user settings
            let replay_user_settings = self.get_user_settings(); // Save all the other user settings
            self.clear(); // Clear the whole grid
            for user_setting in replay_user_settings {
                // Replay all the other user settings
                let result = self.set_value(user_setting.0, user_setting.1, user_setting.2);
                assert!(
                    result.is_ok(),
                    "Unexpected error resetting a user value! Row {}, Column {}, Value {}",
                    user_setting.0,
                    user_setting.1,
                    user_setting.2
                );
                self.grid[user_setting.0][user_setting.1].lock_set_by_user();
            }

            let result = self.run_all_checks();
            assert!(
                result.is_ok(),
                "Unexpected error running checks due to resetting a user value!"
            );
            self.run_extra_checks();
        }
    }

    // Returns false if this value was already set, otherwise returns true to indicate a change
    pub(super) fn set_value(
        &mut self,
        row: usize,
        column: usize,
        value: u8,
    ) -> Result<bool, String> {
        if row >= GRID_SIZE {
            return Err(format!("Invalid row {}", row + 1));
        }
        if column >= GRID_SIZE {
            return Err(format!("Invalid column {}", column + 1));
        }
        if value == 0 || value > 9 {
            return Err(format!("Invalid value {value}"));
        }

        match self.grid[row][column].set_value(value) {
            Err(()) => {
                return Err(format!(
                    "{value} is not a valid entry for row {} column {}",
                    row + 1,
                    column + 1
                ))
            }
            Ok(changed) => return Ok(changed),
        }
    }

    // Prints the current state of the Sudoku grid to the command line
    pub(super) fn print_grid(&self) {
        println!();
        if self.debug_output {
            for (row, row_arr) in self.grid.iter().enumerate() {
                if row != 0 && row % 3 == 0 {
                    println!("{}", "------------+-----------+-----------".bright_cyan());
                }

                for row_index in 0..3 {
                    for (column, val) in row_arr.iter().enumerate() {
                        if column != 0 && column % 3 == 0 {
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
                                        if value == 1 {
                                            chars[0] = '1'
                                        };
                                        if value == 2 {
                                            chars[1] = '2'
                                        };
                                        if value == 3 {
                                            chars[2] = '3'
                                        };
                                    } else if row_index == 1 {
                                        if value == 4 {
                                            chars[0] = '4'
                                        };
                                        if value == 5 {
                                            chars[1] = '5'
                                        };
                                        if value == 6 {
                                            chars[2] = '6'
                                        };
                                    } else if row_index == 2 {
                                        if value == 7 {
                                            chars[0] = '7'
                                        };
                                        if value == 8 {
                                            chars[1] = '8'
                                        };
                                        if value == 9 {
                                            chars[2] = '9'
                                        };
                                    }
                                } else if value == 0 {
                                    chars[0] = '-';
                                    chars[1] = '-';
                                    chars[2] = '-';
                                    if row_index == 0 {
                                        if self.grid[row][column].is_value_valid(1) {
                                            chars[0] = '1'
                                        };
                                        if self.grid[row][column].is_value_valid(2) {
                                            chars[1] = '2'
                                        };
                                        if self.grid[row][column].is_value_valid(3) {
                                            chars[2] = '3'
                                        };
                                    } else if row_index == 1 {
                                        if self.grid[row][column].is_value_valid(4) {
                                            chars[0] = '4'
                                        };
                                        if self.grid[row][column].is_value_valid(5) {
                                            chars[1] = '5'
                                        };
                                        if self.grid[row][column].is_value_valid(6) {
                                            chars[2] = '6'
                                        };
                                    } else if row_index == 2 {
                                        if self.grid[row][column].is_value_valid(7) {
                                            chars[0] = '7'
                                        };
                                        if self.grid[row][column].is_value_valid(8) {
                                            chars[1] = '8'
                                        };
                                        if self.grid[row][column].is_value_valid(9) {
                                            chars[2] = '9'
                                        };
                                    }
                                } else {
                                    if row_index == 0 {
                                        if value == 1 {
                                            chars[0] = '1'
                                        };
                                        if value == 2 {
                                            chars[1] = '2'
                                        };
                                        if value == 3 {
                                            chars[2] = '3'
                                        };
                                    } else if row_index == 1 {
                                        if value == 4 {
                                            chars[0] = '4'
                                        };
                                        if value == 5 {
                                            chars[1] = '5'
                                        };
                                        if value == 6 {
                                            chars[2] = '6'
                                        };
                                    } else if row_index == 2 {
                                        if value == 7 {
                                            chars[0] = '7'
                                        };
                                        if value == 8 {
                                            chars[1] = '8'
                                        };
                                        if value == 9 {
                                            chars[2] = '9'
                                        };
                                    }
                                }
                            }
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
                if row != 0 && row % 3 == 0 {
                    println!("{}", "---------+---------+---------".bright_cyan());
                }

                for (column, val) in row_arr.iter().enumerate() {
                    if column != 0 && column % 3 == 0 {
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
                        }
                    }
                }
                println!();
            }
            println!();
        }
    }
}
