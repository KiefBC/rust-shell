#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::ExitCode;

#[derive(PartialEq)]
enum ShellCommand {
    Exit,
    Echo,
    Unknown,
    Type,
}

impl ShellCommand {
    fn from_str(command: &str) -> Self {
        match command {
            "exit" => ShellCommand::Exit,
            "echo" => ShellCommand::Echo,
            "type" => ShellCommand::Type,
            _ => ShellCommand::Unknown,
        }
    }
}

fn main() -> ExitCode {
    let stdin = io::stdin();
    loop {
        let mut input = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();
        stdin.read_line(&mut input).unwrap();
        let command = parse_command(&input);
        match command {
            ShellCommand::Exit => return handle_exit(),
            ShellCommand::Echo => handle_echo(&input),
            ShellCommand::Type => handle_type(&input),
            ShellCommand::Unknown => invalid_command(&input),
        }
    }
}

fn parse_command(input: &str) -> ShellCommand {
    let mut command_parts = input.split_whitespace();
    match command_parts.next() {
        Some("exit") => ShellCommand::Exit,
        Some("echo") => ShellCommand::Echo,
        Some("type") => ShellCommand::Type,
        _ => ShellCommand::Unknown,
    }
}

fn invalid_command(input: &str) {
    let mut user_input = input.split_whitespace();
    // user_input.next(); // Skip the Type part
    println!("{}: not found", user_input.next().unwrap_or(""),);
}

fn handle_type(input: &str) {
    let mut command_parts = input.split_whitespace();
    command_parts.next(); // Skip the "type" part
    let cmd_name = command_parts.next().unwrap_or("");
    let cmd = ShellCommand::from_str(cmd_name);
    match cmd {
        ShellCommand::Exit => println!("exit is a shell builtin"),
        ShellCommand::Echo => println!("echo is a shell builtin"),
        ShellCommand::Type => println!("type is a shell builtin"),
        ShellCommand::Unknown => {
            if cmd_name.is_empty() {
                println!("No command provided");
            } else {
                invalid_command(cmd_name);
            }
        }
    }
}

fn handle_echo(input: &str) {
    let mut command_parts = input.split_whitespace();
    command_parts.next(); // Skip the "echo" part
    println!("{}", command_parts.collect::<Vec<&str>>().join(" "));
}

fn handle_exit() -> ExitCode {
    // println!("Exiting shell...");
    ExitCode::SUCCESS
}
