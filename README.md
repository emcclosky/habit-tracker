# Habit Tracker

A small command-line application written in Rust to help you track daily habits and streaks.

## 🛠️ Features

- Store habits in a JSON file (`habits.json`)
- Add, list, and mark habits as complete from the terminal
- Track consecutive day streaks per habit
- Simple design for learning Rust

## 🚀 Getting Started

1. **Build & Install**

   ```sh
   cargo build --release
   # optionally install to your PATH:
   cargo install --path .
   ```

2. **Run**

   ```sh
   cargo run -- <command> [args]
   # or if installed:
   habit-tracker <command> [args]
   ```

## 📋 Commands

| Command | Description |
|---|---|
| `add <name>` | Add a new habit |
| `list` | List all habits and their current streaks |
| `complete <name>` | Mark a habit as complete for today |

### Examples

```sh
cargo run -- add "Exercise"
cargo run -- add "Read"
cargo run -- list
cargo run -- complete "Exercise"
```

**Example output for `list`:**

```
Exercise: 3 day streak
Read: 0 day streak
```

## 🔢 Streak Calculation

A streak counts consecutive days a habit was completed. It remains active if you completed the habit **today or yesterday** — missing two or more days resets it to zero.

## 📁 Project Structure

| File | Purpose |
|---|---|
| `main.rs` | Entry point, argument parsing, error handling |
| `command.rs` | CLI command definitions (via clap) |
| `habit.rs` | Habit model, streak logic, error types |
| `storage.rs` | JSON persistence (`habits.json`) |

## 💾 Habits File

Habits are stored in `habits.json` in the project root. The file is created automatically on first use. You can inspect or edit it manually if desired — it uses a straightforward JSON structure.

## 📘 Notes

This project is part of a Rust learning exercise and is intentionally lightweight and straightforward.