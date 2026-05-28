# Agent Workflow Rules

## Cargo Execution Rule (Permanent)

- Always run `cargo` commands from repository root: `C:\Users\blepper\Desktop\sudoku-rust`.
- When a command targets the Rust crate, include `--manifest-path src-tauri/Cargo.toml` instead of changing working directory to `src-tauri`.
- Do not run `cargo` from `src-tauri/`.

