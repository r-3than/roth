# Rothello

Welcome to **Rothello**, a terminal-based implementation of the classic board game Reversi (also known as Othello). This project is built using the Rust programming language, leveraging the `ratatui` and `crossterm` libraries for terminal UI rendering and input handling.

---

## Features

- **Interactive Gameplay**: Navigate the board, make moves, and see the board update in real time.
- **Dynamic Move Highlighting**: Valid moves are displayed for the current player.
- **Chip Flipping Logic**: Automatically flips opponent's chips as per game rules.
- **Score Tracking**: Displays the current score for both players.
- **Game History**: View a scrollable list of all moves played with details.
- **Terminal-based UI**: Minimalist yet functional interface with keyboard controls.

---

## Gameplay Instructions

1. **Navigation**:
   - Move the cursor using the arrow keys (`←`, `↑`, `→`, `↓`).
2. **Make a Move**:
   - Press `Enter` to place a chip at the current cursor position (if it's a valid move).
3. **Scroll Through History**:
   - Use `W` and `S` to scroll up and down through the move history.
4. **Exit the Game**:
   - Press `Q` to quit the game.

---

## Requirements

- **Rust**: Ensure you have the latest version of the Rust programming language installed.
- **Terminal**: A terminal emulator that supports ANSI escape sequences.

---

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/rothello.git
   cd rothello
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the game:
   ```bash
   cargo run --release
   ```

---

## Code Overview

### Main Components

- **App Struct**:
  The main structure managing the game state, including the board, current player, valid moves, and score.

- **Rendering**:
  - The game board is drawn using `Canvas` from `ratatui`.
  - Each chip and valid move is displayed dynamically based on the board state.

- **Game Logic**:
  - `get_moves`: Computes all valid moves for the current player.
  - `flip_chips`: Handles flipping opponent's chips when a move is played.
  - `play`: Updates the board and switches the player after a valid move.

- **User Input Handling**:
  - Handled using the `crossterm` library to capture keypress events and map them to actions.

---

## Controls

| Action         | Key(s)                  |
|----------------|-------------------------|
| Move Cursor    | Arrow Keys (`← ↑ → ↓`) |
| Place Chip     | `Enter`                |
| Scroll History | `W` (Up), `S` (Down)   |
| Quit Game      | `Q`                    |

---

## Future Enhancements

- **AI Opponent**: Add an AI player for single-player mode.
- **Multiplayer**: Enable network-based multiplayer functionality.
- **Improved UI**: Enhance visual feedback and animations in the terminal.

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

## Acknowledgments

- Thanks to the creators of the `ratatui` and `crossterm` libraries for their amazing tools.
- Inspired by the classic board game Othello.

---

Enjoy playing Rothello!

