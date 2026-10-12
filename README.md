# Smart TODO TUI

**A terminal user interface todo application written in Rust. Manage tasks with keyboard shortcuts, store them locally as JSON. No cloud, no accounts, no telemetry.**

## Overview

Smart TODO TUI is a self-contained terminal application. It renders a full interface in your terminal using `ratatui`, handles input via `crossterm`, and persists tasks to a JSON file in the standard user data directory for your operating system. It runs entirely offline and does not depend on any external service.

---

[![Crates.io](https://img.shields.io/crates/v/smart-todo-app-tui)](https://crates.io/crates/smart-todo-app-tui)
[![GitHub top language](https://img.shields.io/github/languages/top/smartlegionlab/smart-todo-app-tui-rs)](https://github.com/smartlegionlab/smart-todo-app-tui-rs)
[![GitHub license](https://img.shields.io/github/license/smartlegionlab/smart-todo-app-tui-rs)](https://github.com/smartlegionlab/smart-todo-app-tui-rs/blob/master/LICENSE)
[![GitHub release](https://img.shields.io/github/v/release/smartlegionlab/smart-todo-app-tui-rs)](https://github.com/smartlegionlab/smart-todo-app-tui-rs/)
[![GitHub stars](https://img.shields.io/github/stars/smartlegionlab/smart-todo-app-tui-rs?style=social)](https://github.com/smartlegionlab/smart-todo-app-tui-rs/stargazers)
[![GitHub forks](https://img.shields.io/github/forks/smartlegionlab/smart-todo-app-tui-rs?style=social)](https://github.com/smartlegionlab/smart-todo-app-tui-rs/network/members)
![Platform](https://img.shields.io/badge/platform-linux%20%7C%20macos%20%7C%20windows-lightgrey)
[![Crates.io Downloads](https://img.shields.io/crates/d/smart-todo-app-tui)](https://crates.io/crates/smart-todo-app-tui)

---

## Features

- Full-screen terminal interface with borders, colors, and highlighted selection
- Add, edit, toggle, and delete tasks with single-key shortcuts
- Two-press delete confirmation to prevent accidental removal
- Modal input for adding and editing task titles
- Long titles are truncated with an ellipsis in the list, and the input
  modal scrolls horizontally so the cursor is always visible
- About screen with version, author, license, and project links
- Automatic persistence to a JSON file after every change
- Cross-platform user data directory handling (Linux, macOS, Windows)
- No external dependencies at runtime, no network access

---

## Key Bindings

| Key             | Action                                   |
|-----------------|------------------------------------------|
| `Up` / `k`      | Move selection up                        |
| `Down` / `j`    | Move selection down                      |
| `Space`         | Toggle done state of the selected task   |
| `Enter`         | Toggle done state of the selected task   |
| `a`             | Add a new task                           |
| `e`             | Edit the selected task                   |
| `d`             | Delete the selected task (press twice)   |
| `i`             | Open the About screen                    |
| `q` / `Esc`     | Quit the application                     |
| `Ctrl+C`        | Quit immediately                         |

Inside the input modal:

| Key         | Action               |
|-------------|----------------------|
| `Enter`     | Save and close       |
| `Esc`       | Cancel and close     |
| `Backspace` | Delete previous char |
| `Left`      | Move cursor left     |
| `Right`     | Move cursor right    |

Inside the About screen:

| Key                         | Action |
|-----------------------------|--------|
| `Esc` / `q` / `i` / `Enter` | Close  |

---

## Command Line

```
smart-todo-app-tui --help
smart-todo-app-tui --version
```

With no arguments the application starts in interactive mode.

---

## Data Storage

Tasks are stored as JSON. The file location depends on your operating system and follows the standard user data directory convention:

| Platform | Path                                                                              |
|----------|-----------------------------------------------------------------------------------|
| Linux    | `~/.local/share/smart-todo-app-tui/todos.json`                                    |
| macOS    | `~/Library/Application Support/com.smartlegionlab.smart-todo-app-tui/todos.json`  |
| Windows  | `%APPDATA%\smartlegionlab\smart-todo-app-tui\data\todos.json`                     |

Format:

```json
[
  {
    "title": "Write documentation",
    "done": false
  },
  {
    "title": "Ship version 0.1.2",
    "done": true
  }
]
```

The file is created automatically on first save. If it does not exist at startup, the application begins with an empty task list.

---

## Build from Source

Requirements:

- Rust 1.88 or newer (edition 2024)
- Cargo

```bash
git clone https://github.com/smartlegionlab/smart-todo-app-tui-rs.git
cd smart-todo-app-tui-rs
cargo build --release
./target/release/smart-todo-app-tui
```

For development:

```bash
cargo run
```

---

## Prebuilt Binaries

Release builds for Linux are published as static musl binaries on the GitHub Releases page:

```
https://github.com/smartlegionlab/smart-todo-app-tui-rs/releases
```

Download the binary, make it executable, and run:

```bash
chmod +x smart-todo-app-tui-<version>-linux-x86_64-musl
./smart-todo-app-tui-<version>-linux-x86_64-musl
```

The musl build is statically linked and runs on any modern Linux distribution without additional dependencies.

---

## Installation (Linux)

### Option 1: Run the prebuilt binary

Download the binary from the Releases page and run it directly. No installation step is required.

### Option 2: Build and install to your home directory

```bash
git clone https://github.com/smartlegionlab/smart-todo-app-tui-rs.git
cd smart-todo-app-tui-rs
cargo build --release
install -Dm755 target/release/smart-todo-app-tui ~/.local/bin/smart-todo-app-tui
```

Ensure `~/.local/bin` is in your `PATH`. Then run:

```bash
smart-todo-app-tui
```

### Option 3: One-command install

```bash
curl -fsSL https://raw.githubusercontent.com/smartlegionlab/smart-todo-app-tui-rs/master/install.sh | bash
```

Installs the binary to `~/.local/bin/` and registers the application in your desktop menu.

### Uninstall

```bash
rm -f ~/.local/bin/smart-todo-app-tui
rm -rf ~/.local/share/smart-todo-app-tui
```

The second command removes your task file. Skip it if you want to keep your tasks.

Or with the uninstall script:

```bash
curl -fsSL https://raw.githubusercontent.com/smartlegionlab/smart-todo-app-tui-rs/master/uninstall.sh | bash
```

---

## Project Layout

```
smart-todo-app-tui-rs/
├── Cargo.toml
├── LICENSE
├── README.md
├── install.sh
├── uninstall.sh
└── src/
    ├── main.rs       Entry point, terminal setup, event loop, CLI parsing
    ├── app.rs        Application state, todo model, mode enum
    ├── events.rs     Keyboard event handling per mode
    ├── ui.rs         Rendering of header, list, footer, modals
    └── storage.rs    JSON persistence and user data directory
```

---

## License

BSD 3-Clause License. See [LICENSE](LICENSE).

Copyright (c) 2026, Alexander Suvorov

---

## Links

- Repository: https://github.com/smartlegionlab/smart-todo-app-tui-rs
- Website: https://smartlegionlab.github.io

