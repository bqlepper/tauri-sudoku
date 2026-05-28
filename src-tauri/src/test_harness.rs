use crate::sudoku_game::Game;
use crate::sudoku_game::sudoku_constants::GRID_SIDE;
use std::fs;
use std::path::Path;

pub struct TestResult {
    pub test_name: String,
    pub success: bool,
    pub message: String,
}

/// Run a single test file
pub fn run_test(test_file_path: &Path) -> TestResult {
    let test_name = test_file_path
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();

    // Read the test file
    let content = match fs::read_to_string(test_file_path) {
        Ok(c) => c,
        Err(e) => {
            return TestResult {
                test_name,
                success: false,
                message: format!("Failed to read file: {}", e),
            }
        }
    };

    // Parse the puzzle
    let entries = parse_puzzle(&content);

    if entries.is_empty() {
        return TestResult {
            test_name,
            success: false,
            message: "No valid puzzle entries found in file".to_string(),
        };
    }

    // Print test header and input
    println!("\n=== Starting Test: {} ===", test_name);
    println!("Input puzzle with {} clues:\n", entries.len());
    println!("{}", content);
    println!();

    // Create a new game
    let mut game = Game::new();

    // Apply all the entries
    for (row, col, value) in entries {
        match game.user_set_value(row, col, value) {
            Ok(_) => {},
            Err(msg) => {
                return TestResult {
                    test_name,
                    success: false,
                    message: format!("Error setting value {} at ({}, {}): {}", value, row + 1, col + 1, msg),
                }
            }
        }
    }

    // Check if the puzzle is solved
    if game.is_solved() {
        // Print the solved puzzle
        game.print_grid();

        TestResult {
            test_name,
            success: true,
            message: "Puzzle solved successfully".to_string(),
        }
    } else {
        TestResult {
            test_name,
            success: false,
            message: "Puzzle not solved after entering all values".to_string(),
        }
    }
}

/// Run all tests in a directory
pub fn run_all_tests(test_dir: &Path) -> Vec<TestResult> {
    let mut results = Vec::new();

    // Read all .txt files in the test directory
    let entries = match fs::read_dir(test_dir) {
        Ok(e) => e,
        Err(e) => {
            results.push(TestResult {
                test_name: "read_dir".to_string(),
                success: false,
                message: format!("Failed to read test directory: {}", e),
            });
            return results;
        }
    };

    for (idx, entry) in entries.enumerate() {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                results.push(TestResult {
                    test_name: format!("directory_read_error_{}", idx),
                    success: false,
                    message: format!("Failed to read directory entry: {}", e),
                });
                continue;
            }
        };

        let path = entry.path();

        // Only process .txt files
        if path.extension().and_then(|s| s.to_str()) == Some("txt") {
            results.push(run_test(&path));
        }
    }

    // Sort results by test name for consistent output
    results.sort_by(|a, b| a.test_name.cmp(&b.test_name));

    results
}

// Private utility functions
/// Parse a sudoku puzzle from the test file format
/// Lines with | are puzzle lines, - means empty, numbers are values
fn parse_puzzle(content: &str) -> Vec<(usize, usize, u8)> {
    let mut entries = Vec::new();
    let mut row = 0;

    for line in content.lines() {
        let line = line.trim();

        // Skip empty lines
        if line.is_empty() {
            continue;
        }

        // Skip separator lines (lines that are all dashes, spaces, and +)
        if line.chars().all(|c| c == '-' || c == '+' || c == ' ') {
            continue;
        }

        // Skip comment lines
        if !line
            .chars()
            .any(|c| c.is_ascii_digit() || c == '-' || c == '|')
        {
            continue;
        }

        // Parse the line
        let mut col = 0;
        for ch in line.chars() {
            if col >= GRID_SIDE {
                break;
            }

            match ch {
                '1'..='9' => {
                    let value = ch as u8 - b'0';
                    entries.push((row, col, value));
                    col += 1;
                }
                '-' | '0' => {
                    // Empty cell
                    col += 1;
                }
                _ => {
                    // Skip spaces, separators, and comments.
                }
            }
        }

        // Only increment row for complete grid lines.
        if col == GRID_SIDE {
            row += 1;
        }

        // Stop after 9 rows
        if row >= GRID_SIDE {
            break;
        }
    }

    entries
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_puzzle() {
        let content = r#" 8  -  - | -  -  - | -  -  -
 -  -  3 | 6  -  - | -  -  -
 -  7  - | -  9  - | 2  -  -
---------+---------+---------
 -  5  - | -  -  7 | -  -  -
 -  -  - | -  4  5 | 7  -  -
 -  -  - | 1  -  - | -  3  -
---------+---------+---------
 -  -  1 | -  -  - | -  6  8
 -  -  8 | 5  -  - | -  1  -
 -  9  - | -  -  - | 4  -  -"#;

        let entries = parse_puzzle(content);

        // Expected entries: (row, col, value) - all in 0-indexed
        let expected = vec![
            (0, 0, 8),   // Row 1: 8 at column 1
            (1, 2, 3),   // Row 2: 3 at column 3
            (1, 3, 6),   // Row 2: 6 at column 4
            (2, 1, 7),   // Row 3: 7 at column 2
            (2, 4, 9),   // Row 3: 9 at column 5
            (2, 6, 2),   // Row 3: 2 at column 7
            (3, 1, 5),   // Row 4: 5 at column 2
            (3, 5, 7),   // Row 4: 7 at column 6
            (4, 4, 4),   // Row 5: 4 at column 5
            (4, 5, 5),   // Row 5: 5 at column 6
            (4, 6, 7),   // Row 5: 7 at column 7
            (5, 3, 1),   // Row 6: 1 at column 4
            (5, 7, 3),   // Row 6: 3 at column 8
            (6, 2, 1),   // Row 7: 1 at column 3
            (6, 7, 6),   // Row 7: 6 at column 8
            (6, 8, 8),   // Row 7: 8 at column 9
            (7, 2, 8),   // Row 8: 8 at column 3
            (7, 3, 5),   // Row 8: 5 at column 4
            (7, 7, 1),   // Row 8: 1 at column 8
            (8, 1, 9),   // Row 9: 9 at column 2
            (8, 6, 4),   // Row 9: 4 at column 7
        ];

        // Check we got exactly 21 entries
        assert_eq!(entries.len(), 21, "Expected 21 entries, got {}", entries.len());

        // Check every single entry matches expected
        for (i, (expected_entry, actual_entry)) in expected.iter().zip(entries.iter()).enumerate() {
            assert_eq!(
                actual_entry, expected_entry,
                "Entry {} mismatch: expected {:?}, got {:?}",
                i, expected_entry, actual_entry
            );
        }

        // Also verify we have exactly the same entries (not just a prefix)
        assert_eq!(entries, expected, "Parsed entries don't match expected entries");
    }

    #[test]
    fn parse_puzzle_treats_zero_as_empty() {
        let content = r#" 0  -  - | -  -  - | -  -  -
 -  -  - | -  -  - | -  -  -
 -  -  - | -  -  - | -  -  -
---------+---------+---------
 -  -  - | -  -  - | -  -  -
 -  -  - | -  -  - | -  -  -
 -  -  - | -  -  - | -  -  -
---------+---------+---------
 -  -  - | -  -  - | -  -  -
 -  -  - | -  -  - | -  -  -
 -  -  - | -  -  - | -  -  -"#;

        let entries = parse_puzzle(content);
        assert!(entries.is_empty());
    }

    #[test]
    fn parse_puzzle_ignores_incomplete_rows() {
        let content = r#" 1  -  - | -  -  - | -  -  -
 -  -  - | -  -  - | -  -
 -  -  - | -  -  - | -  -  -
---------+---------+---------
 -  -  - | -  -  - | -  -  -
 -  -  - | -  -  - | -  -  -
 -  -  - | -  -  - | -  -  -
---------+---------+---------
 -  -  - | -  -  - | -  -  -
 -  -  - | -  -  - | -  -  -
 -  -  - | -  -  - | -  -  -"#;

        let entries = parse_puzzle(content);
        assert_eq!(entries, vec![(0, 0, 1)]);
    }
}
