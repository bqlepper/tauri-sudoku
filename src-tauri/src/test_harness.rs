use crate::sudoku_game::Game;
use std::fs;
use std::path::Path;

pub struct TestResult {
    pub test_name: String,
    pub success: bool,
    pub message: String,
}

/// Parse a sudoku puzzle from the test file format
/// Lines with | are puzzle lines, - means empty, numbers are values
fn parse_puzzle(content: &str) -> Vec<(usize, usize, u8)> {
    let mut entries = Vec::new();
    let mut row = 0;
    
    for line in content.lines() {
        let line = line.trim();
        
        // Skip separator lines and empty lines
        if line.starts_with('-') || line.is_empty() {
            continue;
        }
        
        // Skip comment lines (lines that start with non-digit, non-space, non-|)
        if !line.chars().any(|c| c.is_digit(10) || c == '-' || c == '|') {
            continue;
        }
        
        // Check if this is a puzzle line (contains | or has numbers/dashes)
        if !line.contains('|') && !line.chars().any(|c| c.is_digit(10) || c == '-') {
            continue;
        }
        
        // Parse the line
        let mut col = 0;
        for ch in line.chars() {
            if ch.is_digit(10) {
                let value = ch.to_digit(10).unwrap() as u8;
                entries.push((row, col, value));
                col += 1;
            } else if ch == '-' {
                // Empty cell
                col += 1;
            }
            // Skip spaces, |, and other formatting characters
        }
        
        // Only increment row if we parsed any columns
        if col > 0 {
            row += 1;
        }
        
        // Stop after 9 rows
        if row >= 9 {
            break;
        }
    }
    
    entries
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
    
    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
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
        
        // Check that we found the right number of entries
        assert!(entries.len() > 0);
        
        // Check first entry (row 0, col 0, value 8)
        assert_eq!(entries[0], (0, 0, 8));
    }
}
