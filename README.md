# zed-42-header
Zed plugin to insert 42 header

> ⚠️ **Warning:** This code was written by an A.I. with the goal of replicating the original vim script.

## Prerequisites
Before you begin, ensure you have [Rust and Cargo](https://rustup.rs/) installed on your system to compile the binary.

## Installation

### 1. Configure user
 - Change user and e-mail variables at `main.rs`
```rust
const LOGIN: &str = "USER";
const EMAIL: &str = "E-MAIL@42.fr";
```
- Compile with `cargo build --release`
### 2. Move the file
 Move binary file to `~/.local/bin`

### 3. Add new task in zed
- open command line with `Ctrl+Shift+p`
- write `open tasks` and open tasks.json
- paste this configuration
```json
[
	{
		"label": "42 Header: Insert or Update",
		"command": "~/.local/bin/zed-42-header",
		"args": ["$ZED_FILE"],
		"use_new_terminal": false,
		"allow_concurrent_runs": false,
		"reveal": "never",
	},
]
```
### 4. Add keymap shortcut
- open command line with `Ctrl+Shift+P`
- click on `open keymap`
- click on `Edit in JSON` or `Ctrl+E`
- paste this configuration
```json
[
	{
		"context": "Editor",
		"bindings": {
			"ctrl-alt-h": [
				"task::Spawn",
				{ "task_name": "42 Header: Insert or Update" },
			],
			"f1": ["workspace::SendKeystrokes", "cmd-s ctrl-alt-h"],
			"ctrl-c h": ["workspace::SendKeystrokes", "cmd-s ctrl-alt-h"],
		},
	},
]
```
### 5. Usage
  Press key [F1] or [Ctrl+c h] to use.
