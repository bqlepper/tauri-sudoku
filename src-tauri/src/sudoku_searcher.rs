use chrono::Local;

const MAX_STORED_SOLUTIONS: usize = 10;

// This struct is used for the recursive brute force solution search
pub(super) struct Searcher {
    continue_solution_search: bool, // Brute force search stops whenever this flag goes false.  Set to false by user.  BQL TODO needs work to put brute force search into a thread.
    solution_count: u32, // Counts solutions for each remaining potential value when brute force search is running
    total_solution_count: u32, // Counts all unique solutions when brute force search is running
    grid_changed_by_brute_force: bool, // brute force sets this to true if it changes the grid, which signals to keep brute forcing
    stored_solutions: Vec<Grid>, // Holds a limited number of solutions found by the brute force search
    invalid_values: Vec<(usize, usize, u8)>, // Holds the row, column, and values for invalid values determined by brute force search
}

impl Searcher {
    pub(super) fn new() -> Searcher {
        Searcher {
            continue_solution_search: false,
            grid_changed_by_brute_force: false,
            solution_count: 0,
            total_solution_count: 0,
            stored_solutions: Vec::new(),
            invalid_values: Vec::new(),
        }
    }

    // Called to start a recursive brute force search for a solution and count how many solutoins remain
    // The brute force method also finds invalid values and stores them in the Game::invalid_values vector
    pub(super) fn start_search(&mut self) -> Result<(), &str> {
        if self.grid.is_solved() {
            return Err("No need for search, the puzzle is already solved.");
        }
        self.continue_solution_search = true;
        let mut now = Local::now();
        println!("Starting brute force solution search at: {}", now.format("%Y-%m-%d %H:%M:%S"));
        loop {
            self.solution_count = 0;
            self.total_solution_count = 0;
            self.stored_solutions.clear();
            if !self.continue_solution_search {
                break;
            }
            self.grid_changed_by_brute_force = false;
            self.brute_force_search(Vec::new());
            // Keep rerunning brute force until a single solution is found or no invalid values are discovered
            if self.stored_solutions.len() <= 1 || !self.grid_changed_by_brute_force {
                break;
            }
        }
        now = Local::now();
        println!("\nBrute force solution search completed at: {}", now.format("%Y-%m-%d %H:%M:%S"));
        // If brute force search found only one good solution, stop and print it out
        if self.stored_solutions.len() == 1 {
            self.grid = self.stored_solutions.pop().unwrap();
            self.print_grid();
            return Ok(());
        }

        // Print out all the solutions found by the brute force search up to the MAX_STORED_SOLUTIONS limit
        let mut solution_counter = 1;
        for solution in self.stored_solutions.iter() {
            println!("\n**** Solution {solution_counter} ****");
            solution.print_grid(self.debug_output, &self.user_settings);
            solution_counter += 1;
        }
        if self.stored_solutions.len() == 0 {
            println!("\nThere are no solutions! Something is wrong with the puzzle!");
        } else if self.stored_solutions.len() < MAX_STORED_SOLUTIONS {
            println!("\nThere are {} remaining solutions (see above)", self.stored_solutions.len());
        } else if self.stored_solutions.len() >= MAX_STORED_SOLUTIONS {
            println!("\nThere are {} remaining solutions (see {} of them above)", self.total_solution_count, MAX_STORED_SOLUTIONS);
        } else {
            return Err("\nUnable to find a solution!  Something is wrong with the puzzle!");
        }
        Ok(())
    }

    // I put this in for someone who tries the brute force solution on a puzzle with very few clues and
    // gets tired of how long the brute force method takes and wants to cancel it.
    // BQL TODO However, it will not work until I figure out how to put the brute force in a separate thread
    pub(super) fn stop_search(&mut self) {
        self.continue_solution_search = false;
    }

    // Recursive method to try all remaining potential values and count the number of solutions that are still valid
    // This brute force search also finds invalid values and stores them in the Game::invalid_values vector
    fn brute_force_search (&mut self, trial_settings: Vec<(usize, usize, u8)>) {

        if self.debug_output {
            if trial_settings.len() == 0 {
                print!("*** Entering brute force first time ***");
            } else {
                print!("Entering brute force with: ");
                for setting_entry in &trial_settings {
                    print!("r{}c{}v{}, ", setting_entry.0, setting_entry.1, setting_entry.2);
                }
                println!();
            }
        } else if trial_settings.len() < 3 {
            print!("*");
        }

        // Make a new clean grid with only the user settings
        let mut counter_grid = Grid::new();
        for user_setting in &self.user_settings {
            let result = counter_grid.set_value(user_setting.0, user_setting.1, user_setting.2);
            if !result.is_ok() { return; };  // Something is super wrong, probably the user has put in a bad clue.  Just bail out
        }

        // Add the trial settings working towards a solution to get counted
        for trial_setting in &trial_settings {
            let result =  counter_grid.set_value(trial_setting.0, trial_setting.1, trial_setting.2);
            if !result.is_ok() { return; };  // Something is super wrong, probably the user has put in a bad clue.  Just bail out
        }

        // Remove all the invalid values found by the brute force search
        for invalid_value in &self.invalid_values {
            let result = counter_grid.remove_potential_value(invalid_value.0, invalid_value.1, invalid_value.2);
            if !result.is_ok() { return; };  // Something is super wrong, probably the user has put in a bad clue.  Just bail out
        }

        let result = counter_grid.run_all_checks(false, &self.user_settings); // Not debug for recursive solution search checks.  Change to true for lots of debugging output.
        if let Err(_) = result {
            if self.debug_output {
                println!("\nGot an error on run checks");
            }
            return; // Trial setting can fail, so fail silently and do not count as possible solution
        }

        if self.debug_output {
            println!("\nGrid after running checks");
            counter_grid.print_grid(self.debug_output, &self.user_settings);
        }

        if counter_grid.is_solved() { // If we've found a possible solution
            self.solution_count += 1;

            if self.debug_output {
                if self.stored_solutions.len() < MAX_STORED_SOLUTIONS {
                    println!("\n**** Solution {} ****", self.stored_solutions.len() + 1);
                } else {
                    println!("\n**** Solution Over Maximum ****");
                }
                counter_grid.print_grid(self.debug_output, &self.user_settings);
            }

            if self.stored_solutions.len() < MAX_STORED_SOLUTIONS {
                self.stored_solutions.push(counter_grid);
            }
            return;
        }

        // Find first non-solved cell
        for row in 0..sudoku_grid::GRID_SIZE {
            for column in 0..sudoku_grid::GRID_SIZE {
                if counter_grid.is_cell_solved(row, column) {
                    continue;
                }
                for value in counter_grid.get_values(row, column) {
                    let mut new_trial_settings = trial_settings.clone();
                    new_trial_settings.push((row, column, value));
                    if trial_settings.len() == 0 {
                        self.solution_count = 0;
                    }
                    self.brute_force_search(new_trial_settings);
                    if self.debug_output {
                        println!("Returned from brute force call for row {row}, column {column}, value {value}");
                    }
                    if trial_settings.len() == 0 {
                        self.total_solution_count += self.solution_count;
                        if self.solution_count == 0 {
                            if self.debug_output {
                                println!("\nFound invalid value {value} at row {row}, column {column}!");
                            }
                            self.grid_changed_by_brute_force = true;
                            self.invalid_values.push((row, column, value));
                            match self.grid.remove_potential_value(row, column, value) {
                                Err(_) => return, // Something is super wrong, probably the user has put in a bad clue.  Just bail out
                                Ok(_) => {
                                    let result = self.grid.run_all_checks(self.debug_output, &self.user_settings);
                                    if !result.is_ok() { return; };  // Something is super wrong, probably the user has put in a bad clue.  Just bail out
                                },
                            }
                        }
                    }
                }
                if self.debug_output {
                    println!("Tried all values for row {row}, column {column}, so returning");
                }
                return;
            }
        }
    }
}
