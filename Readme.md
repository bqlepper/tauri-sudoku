# TODOs

- Run with release and see if assertions are causing a lot of the delay
- I made everything public.  Need to evaluate all functions for privacy.
- sudoku_grid.rs has way too many functions, need to break it up.
- Why do cells get bigger when a number is put in.  Stay rigid size, cells

# Getting started

[Link to Tauri getting started page](https://v1.tauri.app/v1/guides/getting-started/setup/)

# To Build and Run in Development Mode

To build and run a Tauri app in Development Mode: cargo tauri dev

## Key Features of Development Mode

- Hot Reloading: Changes to your frontend (HTML, CSS, JavaScript) are automatically reflected without restarting the app.
- Debugging Tools: Developer tools (e.g., Ctrl+Shift+I) are enabled for inspecting the app.
- Unoptimized Build: The app is built in debug mode, meaning it is not optimized for performance or size.
- Console Logs: Logs from both the frontend (console.log) and backend (println!) are visible in the terminal or dev tools.

# To Build and Run in Command Line Only Development Mode

cargo tauri dev -- -- --cli

Note: The double -- might seem weird.  The first one tells the Tauri build program to pass on the command line option to the cargo build program, and the second one tells the carge build program to pass on the command line option to our own Rust main routine.

## Key Features of Command Line Only

This is a special command line option to allow only command line interaction to eliminate the Tauri GUI.  It is good for debugging.

# To Do a Production Build

cargo tauri build

## Key Features of Production Build

- Allows distributing the app to end users.
- Optimized Build: This release mode is optimized for performance and the binary size is minimized.
- No Developer Tools: Developer tools are disabled by default to prevent users from inspecting or modifying the app.
- No Hot Reloading: The app is static and does not reload changes automatically.
- Standalone Executable: The app is packaged as a standalone executable that can be distributed to users.

# Template Info

This app was built using the Tauri + Vanilla template to allow for simple developing with Tauri in vanilla HTML, CSS and Javascript.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
