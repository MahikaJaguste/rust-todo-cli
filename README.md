# rust-todo-cli

A simple command-line todo list manager written in Rust.

---

## Demo

![todo-cli in action](https://github.com/user-attachments/assets/847a474a-f035-4252-b693-5c57f56d1950)

---

## Motivation

This project was created as a way to get started with Rust and learn its ecosystem by building a practical tool. The goal was to explore:

- Rust's module and crate system
- Structs, enums, traits, impl and derive attribute
- Error handling and propagation with `Result` and `Error`
- Command-line argument parsing with [`clap`](https://docs.rs/clap/)
- File I/O and CSV serialization/deserialization
- Writing unit and integration tests

---

## Features

- Add, list, mark as done, remove, clear, and sort todo items
- Stores todos in a CSV file in a user-specific data directory (works on Linux, macOS, Windows)
- Supports priorities (high, medium, low) and status (pending, done)
- Colorful and readable CLI output

---

## Installation

You can install the CLI directly from this GitHub repository using Cargo:

```sh
cargo install --git https://github.com/MahikaJaguste/rust-todo-cli
```

To uninstall:

```sh
cargo uninstall todo
```

---

## Usage

After installation, you can run the CLI from anywhere:

```sh
todo add "Buy milk" high
todo ls
todo done 1
todo rm 2
todo clear
todo sort priority
```

For help on commands and arguments:

```sh
todo --help
todo add --help
todo sort --help
```

---

## Code Structure

- **src/main.rs**  
  Entry point, handles CLI parsing and command dispatch using `clap`.

- **src/lib.rs**  
  Core logic: defines `TodoItem`, `TodoList`, and implements all todo operations.

- **src/csv_io.rs**  
  Handles reading and writing the todo list to a CSV file in a cross-platform data directory.

- **tests/**  
  Integration tests for core functionality.

---
