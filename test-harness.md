# Sudoku Test Harness

This document describes the automated test harness for the Sudoku solver.

## Overview

The test harness automatically:

1. Reads Sudoku puzzle files from the `test/` directory
2. Parses the puzzle format (supports formatted rows with `|`, `.` blanks, and `=` separators)
3. Inputs each value into the solver
4. Verifies that the puzzle is solved correctly
5. Reports pass/fail results

## Test File Format

Test files should be in `.txt` format with the following structure:

```text
 8  .  . | .  .  . | .  .  .
 .  .  3 | 6  .  . | .  .  .
 .  7  . | .  9  . | 2  .  .
============================
 .  5  . | .  .  7 | .  .  .
 .  .  . | .  4  5 | 7  .  .
 .  .  . | 1  .  . | .  3  .
============================
 .  .  1 | .  .  . | .  6  8
 .  .  8 | 5  .  . | .  1  .
 .  9  . | .  .  . | 4  .  .
```

- Numbers (1-9) represent clues
- Dots (`.`) represent empty cells
- Separator lines with `=` are ignored
- `|` is optional and used only for readability
- Comments can be added after the puzzle (they will be ignored)

## Running Tests

There are three ways to run the tests:

### 1. From Headless Mode (Interactive)

```bash
cd src-tauri
cargo run -- --headless
> test
```

This will run all tests in the `test/` directory and display results.

### 2. Standalone Test Runner (Recommended for CI/CD)

```bash
cd src-tauri
cargo run --bin test_runner
```

Or specify a custom test directory:

```bash
cargo run --bin test_runner ../../test
```

The test runner exits with:

- Exit code 0 if all tests pass
- Exit code 1 if any tests fail

### 3. Build and Run Test Runner Binary

```bash
cd src-tauri
cargo build --release --bin test_runner
./target/release/test_runner ../../test
```

## Test Output

Example output:

```text
=== Sudoku Test Runner ===
Test directory: ../../test

=== Test Results ===

[PASS] test1.txt - Puzzle solved successfully
[PASS] test2.txt - Puzzle solved successfully
[PASS] test3.txt - Puzzle solved successfully
[PASS] test4.txt - Puzzle solved successfully
[PASS] test5.txt - Puzzle solved successfully

=== Summary ===
Total:  5
Passed: 5
Failed: 0

All tests passed!
```

If a test fails:

```text
[FAIL] test_bad.txt - Error setting value 5 at (1, 1): 5 is not valid for row 1 column 1!
```

or

```text
[FAIL] test_incomplete.txt - Puzzle not solved after entering all values
```

## Test Results Structure

Each test returns:

- `test_name`: The filename of the test
- `success`: Boolean indicating pass/fail
- `message`: Descriptive message about the result

## Adding New Tests

To add a new test:

1. Create a new `.txt` file in the `test/` directory
2. Format the puzzle using the standard format (with `.` blanks and optional `|` / `=` separators)
3. Run the test harness
4. The new test will automatically be discovered and run

## Implementation Details

### Key Components

1. **`test_harness.rs`**: Core test infrastructure
    - `parse_puzzle()`: Parses test file format into (row, col, value) tuples
    - `run_test()`: Executes a single test file
    - `run_all_tests()`: Runs all tests in a directory
    - `TestResult`: Data structure for test results

2. **`lib.rs`**: Integration with headless mode
    - `run_tests()`: Command handler for "test" command
    - Automatically finds test directory

3. **`bin/test_runner.rs`**: Standalone test runner
    - Can be run independently
    - Proper exit codes for CI/CD integration
    - Command-line argument support

### Design Decisions

- **Public Game API**: Made `Game::new()`, `Game::user_set_value()`, and `Game::is_solved()` public to enable testing
- **Flexible Path Resolution**: Test runner tries multiple paths to find test directory
- **Format Tolerance**: Parser ignores comments and formatting variations
- **Sorted Output**: Tests are run in alphabetical order for consistent results

## CI/CD Integration

Add to your CI/CD pipeline:

```bash
cd src-tauri
cargo test  # Run unit tests
cargo run --bin test_runner  # Run integration tests
```

The test runner returns appropriate exit codes for CI/CD systems.

## Future Enhancements

Possible improvements:

- Support for additional puzzle formats (CSV, JSON)
- Performance benchmarking
- Test coverage reporting
- Parallel test execution
- Expected solution validation (not just "solved" check)
- Difficulty classification
