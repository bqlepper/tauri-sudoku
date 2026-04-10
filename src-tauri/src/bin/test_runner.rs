// Standalone test runner binary
// Run with: cargo run --bin test_runner

use tauri2_sudoku_lib::test_harness;
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Default test directory
    let mut test_dir = "../../test";

    // Allow specifying test directory as argument
    if args.len() > 1 {
        test_dir = &args[1];
    }

    let test_path = Path::new(test_dir);

    if !test_path.exists() {
        eprintln!("Error: Test directory '{}' does not exist", test_dir);
        eprintln!("Usage: {} [test_directory]", args[0]);
        std::process::exit(1);
    }

    println!("=== Sudoku Test Runner ===");
    println!("Test directory: {}\n", test_path.display());

    let results = test_harness::run_all_tests(test_path);

    if results.is_empty() {
        println!("No test files found in directory.");
        return;
    }

    let mut passed = 0;
    let mut failed = 0;

    println!("=== Test Results ===\n");

    for result in &results {
        if result.success {
            println!("✓ {} - {}", result.test_name, result.message);
            passed += 1;
        } else {
            println!("✗ {} - {}", result.test_name, result.message);
            failed += 1;
        }
    }

    println!("\n=== Summary ===");
    println!("Total:  {}", passed + failed);
    println!("Passed: {}", passed);
    println!("Failed: {}", failed);

    if failed == 0 {
        println!("\n🎉 All tests passed!");
        std::process::exit(0);
    } else {
        println!("\n❌ Some tests failed.");
        std::process::exit(1);
    }
}
