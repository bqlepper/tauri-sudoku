# Sudoku Solver

## Purpose

This is a simple sudoku solution solver written with a rust backend and a simple
html, css, javascript UI based on [Tauri](https://v2.tauri.app/). I am playing around with it to learn Rust and to practice using the VSCode copilot AI agent and Palantir Foundry.

## TODOs

- Replace simple html, css, javascript ui with React or something fancy
- Follow Phase 8 implementation/deployment plan: [Phase 8 Web UI + Deployment Plan](docs/plans/phase-8-web-ui-deployment-plan.md)

## Getting started

### Installing Cargo and Rust

#### Installing Cargo and Rust on Ubuntu Linux

Use `rustup`, Rust's official installer which installs `rustc` the rust compiler and `cargo` the build tool and package manager for rust.

- `sudo apt update`
- `sudo apt install build-essential curl -y`
- `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
  - Follow prompts in terminal (just select defaults for everything)
- Configure your cargo environment: `source $HOME/.cargo/env`
- Check `rustc --version` and `cargo --version`

#### Installing Cargo and Rust on Windows or other Linux distros

- In powershell issue this command `winget install Rustlang.Rustup`

#### Installing Cargo and Rust on RedHat based Linux distros

Use `rustup`, Rust's official installer which installs `rustc` the rust compiler and `cargo` the build tool and package manager for rust.

- First some prerequisites
  - `sudo dnf install -y curl gcc gcc-c++ make`
  - `sudo dnf install -y openssl-devel pkgconfig`
- Now get rustup
  - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
  - If the above command fails, you may need to copy the rustup installer to a location where you can run it.

### Installing Tauri

When investigating Tauri, be sure to reference Tauri version 2 documentation instead of the old version 1.
[Link to Tauri getting started page](https://v2.tauri.app/start/)

#### Installing Tauri on Ubuntu Linux

Install prerequisites:

```text
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

```text
    cargo install tauri-cli --version '^2.0.0' --locked
```

Create the starter example Tauri application. Recommend creating the vanilla html, css, javascript version of this initial app to use to for learning and reference.

```text
    cargo install create-tauri-app --locked
    cargo create-tauri-app
```

#### Installing Tauri on Windows

- In powershell install Node.js with `winget install OpenJS.NodeJS.LTS`
- Also in powershell install MS C++ build tools with `winget install Microsoft.VisualStudio.2022.BuildTools`
  - Need to make sure Desktop development with C++ is included
- Use cargo to install tauri cli: `cargo install tauri-cli --version "^2.0.0"`

#### Installing Tauri on RedHat Linux distros

Install prerequisites:

```text
sudo dnf install -y \
  curl \
  wget \
  file \
  openssl-devel \
  libappindicator-gtk3-devel \
  librsvg2-devel \
  patchelf \
  webkit2gtk3-devel \
  gtk3-devel
```

Also: `sudo dnf groupinstall -y "Development Tools"`

Install Tauri CLI:

```text
    cargo install tauri-cli --version '^2.0.0' --locked
```

Note: I ran into a situation where my /tmp drive was mounted with noexec.
So, I had to create a temp directory in my home drive and rerun this cargo install tauri-cli with a different temp directory.
`mkdir -p "$HOME/tmp"`
`TMPDIR="$HOME/tmp" cargo install tauri-cli --version "^2.0.0" --locked`

### Getting the repo from GitHub

Instead of using classic or fine-grain tokens, I chose to use [GitHub CLI](https://cli.github.com/).
So, first install GitHub CLI and do `gh auth login`.
Choose GitHub.com for the 'where you use GitHub' question.
Choose HTTPS for your preferred protocol.
Choose to Authenticate Git with your GitHub credentials.
If you are working on a device where you can use a web browser, choose to authenticate with the web browser login.
If not, you will have to create an SSH key and passphrase.
The title of the SSH key on my GitHub account is "BQL GitHub CLI" (/home/blepper/.ssh/id_ed25519.pub).
Passphrase stored in hint page on my Pixel 8.
Once installed and authenticated, do `gh repo list` and clone with `gh repo clone https://github.com/bqlepper/tauri-sudoku.git`

I have had trouble with authenticating when I have not developed in the github repo for a
long period of time.  To confirm that github cli is still authenticated, you should issue
this command: `gh auth status`
If it is not logged in, you may need to do gh auth login again.
Once you are logged in, resetup automatic authentication by issuing this command: `gh auth setup-git`

### VSCode Plug-Ins

When you open the folder with VSCode, you will probably be prompted to install the tauri and rust-analyzer VSCode plug-ins. Those should be installed.

## Design

For this code the algorithms used to resolve the Sudoku puzzle are an exact cover matrix and the dancing links algorithm.
Refer to these two papers:
    [Sudoku Exact Cover Paper](https://cs.indstate.edu/~bdhome/SUDOKU.pdf)
    [Sudoku Exact Cover Matrix Definition](https://www.stolaf.edu/people/hansonr/sudoku/exactcovermatrix.htm)

## Building and running

Using VSCode, there are tasks setup for the various building and test options.
Review the [tasks.json file](.vscode/tasks.json) for the exact syntax of the commands that are explained below.

### Build Warnings and Errors

#### Bundling warning

A bundling warning occurs because the __TAURI_BUNDLE_TYPE cannot be found.
This symbol is supposed to be generated by tauri-build at compile time and injected into the executable.
But apparently, it is not working.  Research suggests that aligning tauri-cli version with the crate versions
(tauri and tauri-build in Cargo.toml) might fix this and remove the warning.
Or, avoid this warning by turning off bundling: `cargo tauri build --no-bundle`

#### WiX bundling error

On Windows, the build tries to use WiX to create a windows installer.
[WiX GitHub](https://github.com/wixtoolset)
[WiX Fire Giant](https://www.firegiant.com/wixtoolset/)
If you run on a machine that has windows FIPS mode enabled, WiX will fail.  WiX allows some command line -fips options, but Tauri's current bundler does not expose that option directly for use.  If you are in an environment where you can disable Windows FIPS mode, that will resolve this failure.  But if you are working on a corporate controled PC that doesn't allow disabling FIPS mode you are stuck.  And the only way to avoid this error is to turn off bundling: `cargo tauri build --no-bundle`

### Building and running in development mode

To build and run a Tauri app in Development Mode: cargo tauri dev

This generates executables in the build/debug directory.

#### Key Features of Development Mode

- Hot Reloading: Changes to your frontend (HTML, CSS, JavaScript) are automatically reflected without restarting the app.
- Debugging Tools: Developer tools (e.g., Ctrl+Shift+I) are enabled for inspecting the app.
- Unoptimized Build: The app is built in debug mode, meaning it is not optimized for performance or size.
- Console Logs: Logs from both the frontend (console.log) and backend (println!) are visible in the terminal or dev tools.

### Building a production build

cargo tauri build

This generates executables in the build/release directory.

#### Key Features of Production Build

- Allows distributing the app to end users.
- Optimized Build: This release mode is optimized for performance and the binary size is minimized.
- No Developer Tools: Developer tools are disabled by default to prevent users from inspecting or modifying the app.
- No Hot Reloading: The app is static and does not reload changes automatically.
- Standalone Executable: The app is packaged as a standalone executable that can be distributed to users.

### Building and running in headless (command-line only) Development Mode

The debug and release executables can be ran without a UI allowing only command line input and output.

Adding the --headless option to the command line execution runs them in headless mode.

To build in development and run right aferwards, the following command may be used:

cargo tauri dev -- -- --headless

Note: The double -- might seem weird. The first one tells the Tauri build program to pass on the command line option to the cargo build program, and the second one tells the cargo build program to pass on the command line option to our own Rust main routine.

#### Key Features of Command Line Only

This is a special command line option to allow only command line interaction to eliminate the Tauri GUI. It is good for debugging.

### Building and running the Rust built-in unit test

cargo test --manifest-path src-tauri/Cargo.toml

### Building and running the test harness

Refer to the [Test Harness Readme markdown file](test-harness.md) for information on the test harness usage.

### Template Info

This app was built using the Tauri + Vanilla template to allow for simple developing with Tauri in vanilla HTML, CSS and Javascript.

### Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
