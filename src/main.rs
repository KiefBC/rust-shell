#[allow(unused_imports)]
use std::env;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

/// Represents the shell command types.
#[derive(PartialEq, Debug, Clone, Copy)]
enum BuiltinCommand {
    Exit,
    Echo,
    Type,
    Pwd,
}

impl BuiltinCommand {
    /// Attempt to parse a string into `BuiltinCommand`.
    fn from_str(command: &str) -> Option<Self> {
        match command {
            "exit" => Some(BuiltinCommand::Exit),
            "echo" => Some(BuiltinCommand::Echo),
            "type" => Some(BuiltinCommand::Type),
            "pwd" => Some(BuiltinCommand::Pwd),
            _ => None,
        }
    }
}

fn main() -> ExitCode {
    let stdin = io::stdin();

    loop {
        print!("$ ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        match stdin.read_line(&mut input) {
            Ok(0) => break, // Ctrl+D
            Ok(_) => { /* Input received, proceed */ }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                return ExitCode::FAILURE;
            }
        }

        let input = input.trim();
        if input.is_empty() {
            continue; // Skip empty lines
        }

        let mut input_parts = input.split_whitespace();
        let command_name = match input_parts.next() {
            Some(cmd) => cmd,
            None => continue, // Empty input (probably don't need this)
        };
        let args: Vec<&str> = input_parts.collect();

        match BuiltinCommand::from_str(command_name) {
            Some(BuiltinCommand::Exit) => return handle_exit(&args),
            Some(BuiltinCommand::Echo) => handle_echo(&args),
            Some(BuiltinCommand::Type) => handle_type(&args),
            Some(BuiltinCommand::Pwd) => handle_pwd(),
            None => {
                if !try_execute_external(command_name, &args) {
                    print_command_not_found(command_name);
                }
            }
        }
    }
    ExitCode::SUCCESS
}

/// Echo's out the current working directory
fn handle_pwd() {
    let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    handle_echo(&[&current_dir.to_string_lossy()]);
}

/// Helper function to print command not found message.
fn print_command_not_found(cmd_name: &str) {
    println!("{}: command not found", cmd_name);
}

/// Checks if a command is a built-in or an external command.
fn try_execute_external(cmd_name: &str, args: &[&str]) -> bool {
    if let Some(cmd_full_path) = find_in_path(cmd_name) {
        let mut command = Command::new(&cmd_full_path); // Execute using the full path
        command.arg0(cmd_name);
        command.args(args);
        command.stdin(std::process::Stdio::inherit());
        command.stdout(std::process::Stdio::inherit());
        command.stderr(std::process::Stdio::inherit());

        if let Ok(path_var) = env::var("PATH") {
            command.env("PATH", path_var);
        }

        match command.spawn() {
            Ok(mut child) => {
                match child.wait() {
                    Ok(_status) => {
                        // status.success()
                        true
                    }
                    Err(e) => {
                        eprintln!(
                            "Shell: Failed to wait for command '{}': {}",
                            cmd_full_path.display(),
                            e
                        );
                        false
                    }
                }
            }
            Err(e) => {
                eprintln!(
                    "Shell: Failed to execute command '{}': {}",
                    cmd_full_path.display(),
                    e
                );
                false
            }
        }
    } else {
        false // Command not found in PATH
    }
}

/// Checks if a file is executable by checking its permissions.
fn is_executable_file(path: &Path) -> bool {
    match path.metadata() {
        Ok(metadata) => metadata.is_file() && (metadata.permissions().mode() & 0o111) != 0,
        Err(_) => false, // Error reading metadata (e.g., path doesn't exist, permissions)
    }
}

/// Handles the type command.
fn handle_type(args: &[&str]) {
    if args.is_empty() {
        // According to common shell behavior, `type` without arguments doesn't usually print a usage error.
        // It might do nothing or print nothing.
        println!("Usage: type <command>");
        return;
    }
    let cmd_to_type = args[0];

    if BuiltinCommand::from_str(cmd_to_type).is_some() {
        println!("{} is a shell builtin", cmd_to_type);
    } else if let Some(path) = find_in_path(cmd_to_type) {
        println!("{} is {}", cmd_to_type, path.display());
    } else {
        println!("{}: not found", cmd_to_type);
    }
}

/// Finds the command in the PATH environment variable.
fn find_in_path(cmd_name: &str) -> Option<PathBuf> {
    let path_var = env::var("PATH").ok()?; // Returns None if PATH is not set or invalid Unicode

    for dir_str in path_var.split(':') {
        let mut candidate_path = PathBuf::from(dir_str);
        candidate_path.push(cmd_name);

        if is_executable_file(&candidate_path) {
            return Some(candidate_path);
        }
    }
    None
}

/// Handles the echo command.
fn handle_echo(args: &[&str]) {
    println!("{}", args.join(" "));
}

/// Handles the exit command.
fn handle_exit(args: &[&str]) -> ExitCode {
    // The basic `exit` command usually takes an optional exit code.
    // For example, `exit 0` or `exit 1`.
    if args.is_empty() {
        // println!("Exiting shell...");
        ExitCode::SUCCESS
    } else {
        // Try to parse the first argument as an exit code.
        if let Ok(code) = args[0].parse::<u8>() {
            // println!("Exiting shell with code {}...", code);
            ExitCode::from(code)
        } else {
            eprintln!("exit: numeric argument required: {}", args[0]);
            // According to POSIX, if the argument is non-numeric,
            // the shell shall exit with a non-zero exit status, but this is not an error
            // that prevents exit. Some shells exit 255, others 1 or 2.
            ExitCode::from(1)
        }
    }
}
