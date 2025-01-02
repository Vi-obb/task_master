# TaskMstr

## Introduction

"taskmstr" is a simple and efficient command-line task manager I built to practice writing Rust. It helps to  manage tasks directly from the terminal. Built with Rust for performance and reliability, it offers a straightforward interface to add, list, and delete tasks. New Rust learners can use this project to understand the basics of Rust programming, including error handling, file I/O, and command-line argument parsing.

## Features

- Add tasks
- List all tasks with indices
- Delete tasks by index
- Persistent storage of tasks across sessions

## Installation

### Installing from crates.io

To install "taskmstr" from crates.io, run:

```bash
cargo install taskmstr
```

### Building from source

To build "taskmstr" from source, run:

```bash
git clone https://github.com/yourusername/taskmstr.git
cd taskmstr
cargo build --release
```

The binary can be found in the `target/release` directory.

## Usage

### Adding a task

```bash
taskmstr add "learn rust"
```

### Listing all tasks

```bash
taskmstr list
```

### Deleting a task

```bash
taskmstr delete 1
```

## Prerequisites

Rust and Cargo must be installed on your system. Visit [https://rust-lang.org/tools/install](https://rust-lang.org/tools/install) to install Rust.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
