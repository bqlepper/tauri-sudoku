# Purpose

This is a simple soduko solution solver written with a rust backend and a simple
html, css, javascrupt UI based on [Tauri](https://v2.tauri.app/).  I am playing around with it to learn Rust and to practice using the VSCode copilot AI agent.

# TODOs

- Replace simple html, css, javascript ui with React or something fancy
- mergeg tauri 2 stuff into master and update these Readme with what needs to be installed
-   and more about my exact development setup
- Total code cleanup, coding guidelines
- Once the extra checks start running there is a pretty serious delay sometimes,so we should solve that
-     Maybe a background thread but maybe just better user awareness that working is going on
- Try it out on Windows again since it should work cross-platform
- Resize main screen so no scrolling is necessary
- Why do cells get bigger when a number is put in.  Stay rigid size, cells

# Getting started

[Link to Tauri getting started page](https://v2.tauri.app/start/)

# To Build and Run in Development Mode

To build and run a Tauri app in Development Mode: cargo tauri dev

## Key Features of Development Mode

- Hot Reloading: Changes to your frontend (HTML, CSS, JavaScript) are automatically reflected without restarting the app.
- Debugging Tools: Developer tools (e.g., Ctrl+Shift+I) are enabled for inspecting the app.
- Unoptimized Build: The app is built in debug mode, meaning it is not optimized for performance or size.
- Console Logs: Logs from both the frontend (console.log) and backend (println!) are visible in the terminal or dev tools.

# To Build and Run in Command Line Only Development Mode

cargo tauri dev -- -- --cli

Note: The double -- might seem weird.  The first one tells the Tauri build program to pass on the command line option to the cargo build program, and the second one tells the cargo build program to pass on the command line option to our own Rust main routine.

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

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
