# Purpose

This is a simple sudoku solution solver written with a rust backend and a simple
html, css, javascript UI based on [Tauri](https://v2.tauri.app/).  I am playing around with it to learn Rust and to practice using the VSCode copilot AI agent and Palantir Foundry.

# TODOs

- Replace simple html, css, javascript ui with React or something fancy
- Update the Readme with what needs to be installed and exact development setup
- Total code cleanup, coding guidelines
- Once the extra checks start running there is a pretty serious delay sometimes,so we should solve that
    - Maybe a background thread but maybe just better user awareness that working is going on
- Try it out on Windows again since it should work cross-platform
- Why do cells get bigger when a number is put in.  Stay rigid size, cells

# Getting started

## Cargo and Rust

### Installing Cargo and Rust on Ubuntu Linux

Use `rustup`, Rust's official installer which installs `rustc` the rust compiler and `cargo` the build tool and package manager for rust.

- `sudo apt update`
- `sudo apt install build-essential curl -y`
- `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
    - Follow prompts in terminal (just select defaults for everything)
- Configure your cargo environment: `source $HOME/.cargo/env`
- Check `rustc --version` and `cargo --version`

### Installing Cargo and Rust on Windows or other Linux distros

TODO: Need to try this and document

## Tauri

When investigating Tauri, be sure to reference Tauri version 2 documentation instead of the old version 1.
[Link to Tauri getting started page](https://v2.tauri.app/start/)

### Installing Tauri on Ubuntu Linux

Install prerequisites:
```
    sudo apt install libwebkit2gtk-4.1-dev \
        build-essential \
        curl \
        wget \
        file \
        libxdo-dev \
        libssl-dev \
        libayatana-appindicator3-dev \
        librsvg2-dev
```

Install Tauri CLI:
```
    cargo install tauri-cli --version '^2.0.0' --locked
```

Create the starter example Tauri application.  Recommend creating the vanilla html, css, javascript version of this initial app to use to for learning and reference.
```
    cargo install create-tauri-app --locked
    cargo create-tauri-app
```

### Installing Tauri on Windows or other Linux distros

TODO: Need to try this and document

## Getting the repo from GitHub

Instead of using classic or fine-grain tokens, I chose to use [GitHub CLI](https://cli.github.com/) and use an SSH key.  So, first install GitHub CLI and do `gh auth login`.
The title of the SSH key on my GitHub account is "BQL GitHub CLI" (/home/blepper/.ssh/id_ed25519.pub).  Passphrase stored in hint page on my Pixel 8.  Once installed and authenticated, do `gh repo list` and clone with `gh repo clone https://github.com/bqlepper/tauri-sudoku.git`

## VSCode Plug-Ins

When you open the folder with VSCode, you will probably be prompted to install the tauri and rust-analyzer VSCode plug-ins.  Those should be installed.

## Building and running in development mode

To build and run a Tauri app in Development Mode: cargo tauri dev

### Key Features of Development Mode

- Hot Reloading: Changes to your frontend (HTML, CSS, JavaScript) are automatically reflected without restarting the app.
- Debugging Tools: Developer tools (e.g., Ctrl+Shift+I) are enabled for inspecting the app.
- Unoptimized Build: The app is built in debug mode, meaning it is not optimized for performance or size.
- Console Logs: Logs from both the frontend (console.log) and backend (println!) are visible in the terminal or dev tools.

## Building and running in headless (command-line only) Development Mode

TODO: This doesn't work yet and needs to be implemented for developing in remote ssh terminal and for automated testing

cargo tauri dev -- -- --cli

Note: The double -- might seem weird.  The first one tells the Tauri build program to pass on the command line option to the cargo build program, and the second one tells the cargo build program to pass on the command line option to our own Rust main routine.

### Key Features of Command Line Only

This is a special command line option to allow only command line interaction to eliminate the Tauri GUI.  It is good for debugging.

## Building a production build

cargo tauri build

### Key Features of Production Build

- Allows distributing the app to end users.
- Optimized Build: This release mode is optimized for performance and the binary size is minimized.
- No Developer Tools: Developer tools are disabled by default to prevent users from inspecting or modifying the app.
- No Hot Reloading: The app is static and does not reload changes automatically.
- Standalone Executable: The app is packaged as a standalone executable that can be distributed to users.

## Template Info

This app was built using the Tauri + Vanilla template to allow for simple developing with Tauri in vanilla HTML, CSS and Javascript.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
