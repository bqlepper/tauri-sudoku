use crate::sudoku_game::sudoku_constants::GRID_SIDE;
use crate::sudoku_game::Game;
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
            Ok(_) => {}
            Err(msg) => {
                return TestResult {
                    test_name,
                    success: false,
                    message: format!(
                        "Error setting value {} at ({}, {}): {}",
                        value,
                        row + 1,
                        col + 1,
                        msg
                    ),
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
/// Puzzle rows use `.` for empty cells and optional `|` separators.
fn parse_puzzle(content: &str) -> Vec<(usize, usize, u8)> {
    let mut entries = Vec::new();
    let mut row = 0;

    for raw_line in content.lines() {
        let line = raw_line.trim();

        // Skip empty lines and comment lines.
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Skip separator lines (for readability in test files).
        if line.chars().all(|c| c == '=' || c == '+' || c == ' ') {
            continue;
        }

        let normalized = line.replace('|', " ");
        let tokens: Vec<&str> = normalized.split_whitespace().collect();

        // Ignore malformed rows. We only parse complete 9-token puzzle rows.
        if tokens.len() != GRID_SIDE {
            continue;
        }

        let mut row_is_valid = true;
        for (column, token) in tokens.iter().enumerate() {
            match *token {
                "." => {}
                _ => match token.parse::<u8>() {
                    Ok(value) if (1..=9).contains(&value) => entries.push((row, column, value)),
                    _ => {
                        row_is_valid = false;
                        break;
                    }
                },
            }
        }

        if row_is_valid {
            row += 1;
            if row >= GRID_SIDE {
                break;
            }
        }
    }

    entries
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_puzzle() {
        let content = r#"8  .  . | .  .  . | .  .  .
.  .  3 | 6  .  . | .  .  .
.  7  . | .  9  . | 2  .  .
=====================
.  5  . | .  .  7 | .  .  .
.  .  . | .  4  5 | 7  .  .
.  .  . | 1  .  . | .  3  .
=====================
.  .  1 | .  .  . | .  6  8
.  .  8 | 5  .  . | .  1  .
.  9  . | .  .  . | 4  .  ."#;

        let entries = parse_puzzle(content);

        // Expected entries: (row, col, value) - all in 0-indexed
        let expected = vec![
            (0, 0, 8), // Row 1: 8 at column 1
            (1, 2, 3), // Row 2: 3 at column 3
            (1, 3, 6), // Row 2: 6 at column 4
            (2, 1, 7), // Row 3: 7 at column 2
            (2, 4, 9), // Row 3: 9 at column 5
            (2, 6, 2), // Row 3: 2 at column 7
            (3, 1, 5), // Row 4: 5 at column 2
            (3, 5, 7), // Row 4: 7 at column 6
            (4, 4, 4), // Row 5: 4 at column 5
            (4, 5, 5), // Row 5: 5 at column 6
            (4, 6, 7), // Row 5: 7 at column 7
            (5, 3, 1), // Row 6: 1 at column 4
            (5, 7, 3), // Row 6: 3 at column 8
            (6, 2, 1), // Row 7: 1 at column 3
            (6, 7, 6), // Row 7: 6 at column 8
            (6, 8, 8), // Row 7: 8 at column 9
            (7, 2, 8), // Row 8: 8 at column 3
            (7, 3, 5), // Row 8: 5 at column 4
            (7, 7, 1), // Row 8: 1 at column 8
            (8, 1, 9), // Row 9: 9 at column 2
            (8, 6, 4), // Row 9: 4 at column 7
        ];

        // Check we got exactly 21 entries
        assert_eq!(
            entries.len(),
            21,
            "Expected 21 entries, got {}",
            entries.len()
        );

        // Check every single entry matches expected
        for (i, (expected_entry, actual_entry)) in expected.iter().zip(entries.iter()).enumerate() {
            assert_eq!(
                actual_entry, expected_entry,
                "Entry {} mismatch: expected {:?}, got {:?}",
                i, expected_entry, actual_entry
            );
        }

        // Also verify we have exactly the same entries (not just a prefix)
        assert_eq!(
            entries, expected,
            "Parsed entries don't match expected entries"
        );
    }

    #[test]
    fn parse_puzzle_treats_dot_as_empty() {
        let content = r#".  .  . | .  .  . | .  .  .
.  .  . | .  .  . | .  .  .
.  .  . | .  .  . | .  .  .
=====================
.  .  . | .  .  . | .  .  .
.  .  . | .  .  . | .  .  .
.  .  . | .  .  . | .  .  .
=====================
.  .  . | .  .  . | .  .  .
.  .  . | .  .  . | .  .  .
.  .  . | .  .  . | .  .  ."#;

        let entries = parse_puzzle(content);
        assert!(entries.is_empty());
    }

    #[test]
    fn parse_puzzle_ignores_incomplete_rows() {
        let content = r#"1  .  . | .  .  . | .  .  .
.  .  . | .  .  . | .  .
.  .  . | .  .  . | .  .  .
=====================
.  .  . | .  .  . | .  .  .
.  .  . | .  .  . | .  .  .
.  .  . | .  .  . | .  .  .
=====================
.  .  . | .  .  . | .  .  .
.  .  . | .  .  . | .  .  .
.  .  . | .  .  . | .  .  ."#;

        let entries = parse_puzzle(content);
        assert_eq!(entries, vec![(0, 0, 1)]);
    }
}
