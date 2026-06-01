---
name: cross-platform-hygiene-rust
description: Enforce Linux/Windows hygiene for Rust repositories by checking LF line endings, tab/trailing whitespace, case-sensitive path collisions, OS-specific path assumptions, and running Rust test commands (cargo unit tests and test harness). Use when preparing commits or auditing portability regressions.
---

# Cross Platform Hygiene Rust

## Overview

Use this skill to audit and fix cross-platform and formatting issues in this Rust repo before commit.

## Run The Audit

1. Run `scripts/check-hygiene.ps1`.
2. Review reported issues.
3. If whitespace cleanup is needed, run `scripts/fix-whitespace.ps1`.
4. Re-run `scripts/check-hygiene.ps1` to confirm all checks pass.

## Validation Commands

Run from repository root:

1. `cargo test --manifest-path src-tauri/Cargo.toml`
2. `cargo run --manifest-path src-tauri/Cargo.toml --bin test_runner -- test`

Prefer running `scripts/check-hygiene.ps1 -RunTests` to execute all checks and validations in one command.

## Expected Policies

1. Text files use LF line endings.
2. No tab characters in tracked text files.
3. No trailing whitespace.
4. Prefer 4-space indentation across source/config/docs in this repo.
5. Avoid hardcoded stale repo paths or OS-specific assumptions in source/docs.
6. Keep paths case-consistent for Linux case-sensitive filesystems.

