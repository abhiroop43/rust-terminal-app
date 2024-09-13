# Terminal UI App with Ratatui

This project is a terminal-based user interface (TUI) application built using **Ratatui** (a fork of TUI-rs) in Rust. It features a visually interactive list where users can navigate through items using arrow keys, with selected items highlighted.

## Features

- Navigate through a list of items using the **up** and **down** arrow keys.
- Visual feedback for the selected item (highlighted in bold and yellow).
- Cross-platform compatibility using **Crossterm** for terminal handling.
- Clean, responsive terminal UI built with **Ratatui**.

## Requirements

- **Rust** (1.56 or higher)
- **Cargo-edit** (for managing dependencies via `cargo add`)

Install `cargo-edit` with:

```bash
cargo install cargo-edit
```

## Installation

1. Clone the repository:

```bash
git clone git@github.com:abhiroop43/rust-terminal-app.git
cd terminal-ui-app
```

2. Add dependencies (if not already in Cargo.toml):

```bash
cargo add ratatui
```

3. Build the project:

```bash
cargo build
```

4. Run the application:

```bash
cargo run
```

## Usage

Once the application is running:

- Use the up (↑) and down (↓) arrow keys to navigate through the list of items.
- The currently selected item will be highlighted in yellow and bold for visual feedback.
- Press q to exit the application.

## File Structure

```less
src/
  ├── app_state.rs     // Manages application state (items, selection logic)
  ├── main.rs          // Main entry point and UI logic
Cargo.toml             // Project dependencies
```

`app_state.rs`
This file manages the application state, including the list of items and the currently selected item. It provides methods to navigate through the list (e.g., `next()` and `previous()`).

`main.rs`
This file contains the main logic for rendering the UI and handling input. It sets up the terminal using `Crossterm` and `Ratatui`, draws the UI with blocks and lists, and listens for user input to update the selection.

## Dependencies

Ratatui: A Rust library for building rich terminal user interfaces.
Crossterm: Cross-platform library for handling terminal I/O.
Rust: Programming language for safe and fast system-level development.

## Contributing

If you'd like to contribute to this project, feel free to submit issues or pull requests!

## License

This project is licensed under the MIT License.
