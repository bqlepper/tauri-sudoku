# Exact Cover + DLX Refactor Plan

## Scope

Refactor Sudoku solving logic to:

1. Use an exact cover matrix with Sudoku's 4 constraint families.
2. Use Donald Knuth's Dancing Links (DLX) algorithm for search.
3. Preserve current UI/headless command behavior while replacing solver internals.

This plan implements the first TODO in `Readme.md`.

## Preconditions

1. Work on branch `feature/exact-cover-refactor`.
2. Enforce Linux-friendly line endings:
    - `.gitattributes`: `* text=auto eol=lf`
    - repo-local Git config: `core.autocrlf=false`, `core.eol=lf`, `core.safecrlf=true`

## Status Model (Mutually Exclusive for Solution Count)

Use one of these solution-count states for a given board:

1. `Contradiction`: no valid completion exists (`0` solutions).
2. `Unique`: exactly one valid completion exists.
3. `MultipleBelowCap(n)`: at least two and fewer than `cap` completions exist.
4. `AtLeastCap(cap)`: solver found `>= cap` completions and stopped early.

Board progress is tracked separately:

1. `SolvedNow`: board currently has 81 valid assignments.
2. `UnsolvedNow`: board not fully assigned yet.

## Exact Cover Model

Sudoku exact cover dimensions:

1. Rows: `9 x 9 x 9 = 729` assignment candidates `(row, col, value)`.
2. Columns: `9 x 9 x 4 = 324` constraints:
    - Cell occupancy constraint
    - Row-value uniqueness constraint
    - Column-value uniqueness constraint
    - Box-value uniqueness constraint

Each assignment row sets exactly 4 columns (one in each family).

## Implementation Phases

1. Add exact cover + DLX module under `src-tauri/src/sudoku_game/`.
2. Implement mapping helpers:
    - assignment `(r,c,v)` <-> exact-cover row index
    - each of the 4 constraint families <-> column index
3. Implement DLX engine using memory-managed Rust containers (index-based vectors; no raw pointers/unsafe).
4. Build analyzer API:
    - apply user clues
    - detect contradictions
    - compute candidate validity per cell
    - count solutions with cap and early-stop
5. Integrate into existing game/grid APIs without changing frontend contract.
6. Retire old check/trial-based solver path after parity is verified.
7. Add tests for mapping invariants, contradiction handling, and cap behavior.
8. Update documentation (README + architecture doc) after implementation stabilizes.

## Integration Targets

Expected primary integration files:

1. `src-tauri/src/sudoku_game.rs`
2. `src-tauri/src/sudoku_game/sudoku_grid.rs`
3. New module(s) in `src-tauri/src/sudoku_game/` for exact cover/DLX
4. Existing tests in `src-tauri/src/test_harness.rs` plus new solver tests

Legacy solver paths expected to be replaced:

1. `src-tauri/src/sudoku_game/sudoku_grid/sudoku_grid_checks.rs`
2. `src-tauri/src/sudoku_game/sudoku_grid/sudoku_grid_trials.rs`

## Validation Plan

1. Unit tests for exact-cover mapping:
    - 729 rows, 324 columns
    - each row covers exactly 4 constraints
2. Unit tests for contradiction detection from conflicting clues.
3. Unit tests for `count_solutions` with cap:
    - `0` case -> `Contradiction`
    - `1` case -> `Unique`
    - `2+` but `< cap` -> `MultipleBelowCap(n)`
    - `>= cap` -> `AtLeastCap(cap)`
4. Run current harness tests in `test/*.txt` and ensure no behavior regression.

## Phase 7 Completion Notes

Cleanup and test expansion work completed:

1. Hardened command/API boundaries in `lib.rs`:
    - Centralized parsing helpers for row/column/value.
    - Added `cell_index` bounds validation for value/delete commands.
    - Removed panic-prone `unwrap()` calls in headless input/output and mutex lock path.
2. Simplified data structures:
    - Replaced manual `Cell`/`Grid` cloning with derived `Clone` (and `Copy` for `Cell`).
    - Simplified grid initialization.
3. Improved runtime resilience:
    - Replaced several panic assertions in replay/delete flow with error logging + early return.
4. Added tests:
    - New `lib.rs` tests for parser and command boundary behavior.
    - New `sudoku_grid.rs` tests for bounds checking and direct conflict detection.
    - New parser edge-case tests in `test_harness.rs`.

Pending local verification on a machine with Rust toolchain installed:

1. `cargo test --manifest-path src-tauri/Cargo.toml`
2. `cargo run --manifest-path src-tauri/Cargo.toml --bin test_runner`

## Security and Reliability Notes

1. Treat all UI/headless inputs as untrusted; validate row/col/value bounds at API boundaries.
2. Do not log secrets or sensitive data.
3. Return user-safe error strings; avoid leaking internal details.
4. Keep implementation explicit and reviewable; avoid broad drive-by refactors.

## Risks and Mitigations

1. Incorrect mapping formulas:
    - Mitigate with exhaustive mapping tests and assertions.
2. DLX cover/uncover bugs:
    - Mitigate with invariant checks in tests and strict index validation.
3. UX/performance regressions:
    - Mitigate with early-stop cap and compatibility checks with existing UI flows.

## Reference Links

1. [Sudoku exact cover matrix Indiana State whitepaper](https://cs.indstate.edu/~bdhome/SUDOKU.pdf)
2. [Exact cover matrix and dancing links whitepaper](https://www.stolaf.edu/people/hansonr/sudoku/exactcovermatrix.htm)
