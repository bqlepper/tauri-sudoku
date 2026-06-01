# Checklist

Use this checklist when running cross-platform hygiene work:

1. Run the audit script from repo root.
2. Confirm no tabs and no trailing whitespace in tracked text files.
3. Confirm tracked text files are LF.
4. Confirm no case-colliding tracked paths.
5. Review portability-smell matches and decide whether each is acceptable.
6. Run Rust test commands:
    - `cargo test --manifest-path src-tauri/Cargo.toml`
    - `cargo run --manifest-path src-tauri/Cargo.toml --bin test_runner -- test`
7. Summarize findings and exact files changed.

