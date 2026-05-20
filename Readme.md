# Purpose

This is a simple sudoku solution solver written with a rust backend and a simple
html, css, javascript UI based on [Tauri](https://v2.tauri.app/). I am playing around with it to learn Rust and to practice using the VSCode copilot AI agent and Palantir Foundry.

# TODOs

- Refactor logic to use exact cover matrix with 4 constraints:
    1. A cell can only contain one integer 1-9
    2. Each row must contain each integer 1-9 only once
    3. Each column must contain each integer 1-9 only once
    4. Each of the 9 9x9 boxes of cells must contain each integer 1-9 only once
  This will result in an exact cover matrix of 9x9x9=729 rows and 9x9x4=324 columns.
  The refactor should also use Donald Knuth's Dancing Links algorithm to eliminate all
  invalid values as the user enters clues and to count the number of remaining solutions.
  Refer to these two papers:
  https://cs.indstate.edu/~bdhome/SUDOKU.pdf
  https://www.stolaf.edu/people/hansonr/sudoku/exactcovermatrix.htm
- Replace simple html, css, javascript ui with React or something fancy
- Total code cleanup, coding guidelines
- Once the extra checks start running there is a pretty serious delay sometimes,so we should solve that
  - Maybe a background thread but maybe just better user awareness that working is going on
- Try it out on Windows again since it should work cross-platform

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

Create the starter example Tauri application. Recommend creating the vanilla html, css, javascript version of this initial app to use to for learning and reference.

```
    cargo install create-tauri-app --locked
    cargo create-tauri-app
```

### Installing Tauri on Windows or other Linux distros

TODO: Need to try this and document

## Getting the repo from GitHub

Instead of using classic or fine-grain tokens, I chose to use [GitHub CLI](https://cli.github.com/) and use an SSH key. So, first install GitHub CLI and do `gh auth login`.
The title of the SSH key on my GitHub account is "BQL GitHub CLI" (/home/blepper/.ssh/id_ed25519.pub). Passphrase stored in hint page on my Pixel 8. Once installed and authenticated, do `gh repo list` and clone with `gh repo clone https://github.com/bqlepper/tauri-sudoku.git`

I have had trouble with authenticating when I have not developed in the github repo for a
long period of time.  To confirm that github cli is still authenticated, you should issue
this command: `gh auth status`
If it is not logged in, you may need to do gh auth login again.
Once you are logged in, resetup automatic authentication by issuing this command: `gh auth setup-git`

## VSCode Plug-Ins

When you open the folder with VSCode, you will probably be prompted to install the tauri and rust-analyzer VSCode plug-ins. Those should be installed.

## Building and running in development mode

To build and run a Tauri app in Development Mode: cargo tauri dev

This generates executables in the build/debug directory.

### Key Features of Development Mode

- Hot Reloading: Changes to your frontend (HTML, CSS, JavaScript) are automatically reflected without restarting the app.
- Debugging Tools: Developer tools (e.g., Ctrl+Shift+I) are enabled for inspecting the app.
- Unoptimized Build: The app is built in debug mode, meaning it is not optimized for performance or size.
- Console Logs: Logs from both the frontend (console.log) and backend (println!) are visible in the terminal or dev tools.

## Building a production build

cargo tauri build

This generates executables in the build/release directory.

### Key Features of Production Build

- Allows distributing the app to end users.
- Optimized Build: This release mode is optimized for performance and the binary size is minimized.
- No Developer Tools: Developer tools are disabled by default to prevent users from inspecting or modifying the app.
- No Hot Reloading: The app is static and does not reload changes automatically.
- Standalone Executable: The app is packaged as a standalone executable that can be distributed to users.

## Building and running in headless (command-line only) Development Mode

The debug and release executables can be ran without a UI allowing only command line input and output.

Adding the --headless option to the command line execution runs them in headless mode.

To build in development and run right aferwards, the following command may be used:

cargo tauri dev -- -- --headless

Note: The double -- might seem weird. The first one tells the Tauri build program to pass on the command line option to the cargo build program, and the second one tells the cargo build program to pass on the command line option to our own Rust main routine.

### Key Features of Command Line Only

This is a special command line option to allow only command line interaction to eliminate the Tauri GUI. It is good for debugging.

## Template Info

This app was built using the Tauri + Vanilla template to allow for simple developing with Tauri in vanilla HTML, CSS and Javascript.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
