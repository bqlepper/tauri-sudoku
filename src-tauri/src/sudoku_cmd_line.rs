use crate::sudoku_game::Game;

// Puzzled and Puzzle have the same functionality, but Puzzle uses a GUI for user interaction and
// Puzzled uses only the command line for user interaction.  The 'd' on the end stands for debug because that is how I roll.
pub(super) struct Puzzled {
    game: Game,
}

impl Puzzled
{
    pub(super) fn new() -> Puzzled {
        Puzzled {
            game: Game::new(),
        }
    }
}

impl Puzzled {
    pub(super) fn go(&mut self) {
        // This loop allows users to enter given puzzle values from the command line
        loop {
            println!("\nEnter row, column, and value separated by spaces (h for help):");

            // Read a line from standard input
            let mut input_line = String::new();
            match std::io::stdin().read_line(&mut input_line) {
                Ok(_) => {
                    let input_numbers: Vec<&str> = input_line.trim().split_whitespace().collect();

                    if input_numbers.len() == 1 {
                        match input_numbers[0].to_lowercase().as_str() {
                            "q" | "quit" => {
                                println!("Goodbye.");
                                break;
                            }
                            "d" | "debug" => {
                                self.game.set_debug(true);
                                self.game.print_grid();
                                continue;
                            }
                            "o" | "off" => {
                                self.game.set_debug(false);
                                self.game.print_grid();
                                continue;
                            }
                            "c" | "clear" => {
                                self.game.clear();
                                self.game.print_grid();
                                continue;
                            }
                            "h" | "help" => {
                                println!("\nDuring play the following keypresses are allowed:");
                                println!("\n  1-9              Enter sudoku puzzle clues");
                                println!("  del or backspace   Clear a puzzle clue that was already entered");
                                println!("  c (clear)          Clear the entire puzzle");
                                println!("  s (search)         Runs brute force solution search (can take a long time if too few clues provided)");
                                println!("  q (quit)           Exit the program (for command line only)");
                                println!("  d (debug)          Turn on debugging output on the command line");
                                println!("  o (off)            Turn off debugging output on the command line\n");
                                println!("For command line enter row column and value on one line separated by spaces with values 1-9 for all.\n");
                                println!("For GUI the mouse, arrow keys, and tab can be used to navigate the grid and use red window X to close.\n");
                                continue;
                            }
                            "s" | "search" => {
                                let result = self.game.start_search();
                                if let Err(out_message) = result {
                                    println!("{out_message}");
                                }
                                continue;
                            }
                            "b" | "break" => { // BQL TODO this will not work until I figure out how to run the brut force search in a thread that can die when the user selects to stop the search.
                                self.game.stop_search();
                                continue;
                            }
                            _ => {
                                println!("Bad input, try again (or type 'help'): {}", input_line);
                                continue;
                            }
                        }
                    }

                    if input_numbers.len() < 3 {
                        println!("Bad input, try again (too few input vaules): {}", input_line);
                        continue;
                    } else if input_numbers.len() > 3 {
                        println!("Warning: Ignoring extra input values");
                    }

                    let row = match input_numbers[0].parse::<usize>() {
                        Ok(row_num) => row_num,
                        Err(_) => 0,
                    };

                    let column = match input_numbers[1].parse::<usize>() {
                        Ok(column_num) => column_num,
                        Err(_) => 0,
                    };

                    if row < 1 || column < 1 {
                        println!("Row & column must be greater than zero");
                        continue;
                    }

                    let value = match input_numbers[2].parse::<u8>() {
                        Ok(value_num) => value_num,
                        Err(_) => 0,
                    };

                    match self.game.user_set_value(row-1, column-1, value) {
                        Ok(_) => {
                            self.game.print_grid();
                        },
                        Err(user_msg) => {
                            println!("Bad input: {user_msg}");
                            continue;
                        }
                    }
                },
                Err(_) => {
                    println!("Bad input (error reading line): {}", input_line);
                    break;
                }
            }
        }
    }
}