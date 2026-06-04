# Always Show Solution Count Plan

## Goals

1. Always show remaining solution count in the UI.
2. Remove legacy count command path (`11`) from the Tauri UI command interface.
3. Remove `count` command from headless CLI help/dispatcher.
4. Remove `Search` button and related keyboard/UI behavior.
5. Replace numeric `user_input` command protocol with explicit typed request payloads.

## Scope

- Backend: `src-tauri/src/lib.rs`, `src-tauri/src/sudoku_game.rs`, `src-tauri/src/sudoku_game/sudoku_grid.rs`, and `src-tauri/src/sudoku_game/sudoku_grid/sudoku_grid_trials.rs`
- Frontend: `src-ui/index.html`, `src-ui/main.js`, `src-ui/styles.css`
- Tests: update Rust unit tests impacted by API and behavior changes.

## Design

### 1. Typed request/response contract for `user_change`

- Replace `(cell_index, user_input)` command arguments with a single JSON request object.
- Use a tagged enum for actions:
  - `set_cell { cell_index, value }`
  - `clear_cell { cell_index }`
  - `clear_grid`
  - `set_debug { enabled }`
- Return structured grid data directly from Rust instead of JSON-encoded strings.

### 2. Always include remaining solution summary

- Extend grid response payload with:
  - `remainingSolutionsCount`
  - `remainingSolutionsAtLeastCap`
  - `remainingSolutionsText`
- Compute using existing exact-cover solver cap (`50`) for every returned grid snapshot.
- Display exact count when below cap, and `At least 50` when capped.

### 3. Remove legacy command paths

- Delete command `11` count behavior from Tauri command handler.
- Remove `count` branch from headless CLI mode and help output.
- Remove stale comments/docs describing numeric command inputs.

### 4. UI changes

- Remove `Search` button markup and JavaScript event wiring.
- Remove keyboard shortcut mapping that triggered search (`s`/`S`).
- Add persistent UI line for remaining solution count.
- Keep game status/error messaging independent from solution-count display.

## Verification

1. Run Rust tests: `cargo test --manifest-path src-tauri/Cargo.toml`
2. Manual UI checks:
   - Enter clues and confirm solution count updates each action.
   - Delete clues and confirm count updates.
   - Clear grid and confirm count resets/updates.
   - Force contradiction and confirm count shows `0`.
   - Solve puzzle and confirm solved message and remaining count behavior are coherent.

## Risks

- Always-count behavior may increase per-keystroke compute cost on very open grids.
- API migration requires frontend and backend updates to remain synchronized.
