# Rust Shell

A POSIX-compliant shell implementation written in Rust, built as part of the CodeCrafters "Build Your Own Shell" challenge.

## What This Shell Does

This shell provides an interactive command-line interface that can:
- Execute built-in commands
- Run external programs from the system PATH
- Handle command parsing with proper quote handling
- Provide a REPL (Read-Eval-Print Loop) experience

## Current Features

### Built-in Commands
- **`exit [code]`** - Exit the shell with optional exit code
- **`echo <text>`** - Print text to stdout, supports quoted strings
- **`pwd`** - Print current working directory
- **`cd <path>`** - Change directory, supports `~` for home directory
- **`type <command>`** - Show whether a command is built-in or external, and its location

### Shell Features
- **Command parsing** - Handles single-quoted strings and whitespace properly
- **External command execution** - Finds and executes programs from system PATH
- **Error handling** - Proper error messages for missing commands and directories
- **Interactive prompt** - Displays `$ ` prompt and handles Ctrl+D for exit

## How to Run

### Prerequisites
- Rust 1.80 or later
- Cargo package manager

### Running the Shell

1. **Compile and run directly with Cargo:**
   ```sh
   cargo run
   ```

2. **Using the provided script:**
   ```sh
   ./your_program.sh
   ```

3. **Build and run the executable:**
   ```sh
   cargo build --release
   ./target/release/codecrafters-shell
   ```

### Example Usage

```sh
$ echo "Hello, World!"
Hello, World!
$ pwd
/Users/kiefer/programming/codecrafters/rust-shell
$ cd ~
$ type echo
echo is a shell builtin
$ type ls
ls is /bin/ls
$ ls
Cargo.toml  README.md  src/  your_program.sh
$ exit 0
```

## Development

Built with CodeCrafters challenge requirements in mind, focusing on:
- POSIX compliance
- Proper error handling
- Clean, readable Rust code
- Educational shell implementation

**Note**: This project is part of the CodeCrafters learning platform. Visit [codecrafters.io](https://codecrafters.io) to try the challenge yourself.