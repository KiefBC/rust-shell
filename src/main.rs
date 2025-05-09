#[allow(unused_imports)]
use std::env;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

#[derive(PartialEq)]
enum ShellCommand {
    Exit,
    Echo,
    Unknown,
    Type,
    Executable,
}

impl ShellCommand {
    fn from_str(command: &str) -> Self {
        match command {
            "exit" => ShellCommand::Exit,
            "echo" => ShellCommand::Echo,
            "type" => ShellCommand::Type,
            "executable" => ShellCommand::Executable,
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
        if stdin.read_line(&mut input).unwrap() == 0 {
            break;
        }
        let input = input.trim_end();
        let mut parts = input.split_whitespace();
        let cmd_name = match parts.next() {
            Some(cmd) => cmd,
            None => continue, // Empty input
        };
        let args: Vec<&str> = parts.collect();
        let command = ShellCommand::from_str(cmd_name);

        match command {
            ShellCommand::Exit => return handle_exit(),
            ShellCommand::Echo => handle_echo(&args),
            ShellCommand::Type => handle_type(&args),
            ShellCommand::Executable => {
                println!("{} is a shell builtin", cmd_name);
                continue;
            }
            ShellCommand::Unknown => {
                if !try_execute_external(cmd_name, &args) {
                    invalid_command(cmd_name);
                }
            }
        }
    }
    ExitCode::SUCCESS
}

fn try_execute_external(cmd_name: &str, args: &[&str]) -> bool {
    let path_var = match env::var("PATH") {
        Ok(val) => val,
        Err(_) => return false,
    };
    for dir in path_var.split(':') {
        let mut cmd_path = PathBuf::from(dir);
        cmd_path.push(cmd_name);
        if cmd_path.is_file() && is_executable(&cmd_path) {
            // Found executable, run it by name, let OS search $PATH
            let mut child = match Command::new(cmd_name)
                .args(args)
                .stdin(std::process::Stdio::inherit())
                .stdout(std::process::Stdio::inherit())
                .stderr(std::process::Stdio::inherit())
                .env("PATH", &path_var) // ensure correct PATH
                .spawn()
            {
                Ok(child) => child,
                Err(_) => return false,
            };
            let _ = child.wait();
            return true;
        }
    }
    false
}

fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    match path.metadata() {
        Ok(meta) => (meta.permissions().mode() & 0o111) != 0,
        Err(_) => false,
    }
}

fn invalid_command(input: &str) {
    let mut user_input = input.split_whitespace();
    // user_input.next(); // Skip the Type part
    println!("{}: not found", user_input.next().unwrap_or(""));
}

fn handle_type(args: &[&str]) {
    if args.is_empty() {
        println!("No command provided");
        return;
    }
    let cmd_name = args[0];
    let cmd = ShellCommand::from_str(cmd_name);
    match cmd {
        ShellCommand::Exit => println!("exit is a shell builtin"),
        ShellCommand::Echo => println!("echo is a shell builtin"),
        ShellCommand::Type => println!("type is a shell builtin"),
        ShellCommand::Executable => {}
        ShellCommand::Unknown => {
            // Search PATH for the external command
            if let Some(found_path) = find_in_path(cmd_name) {
                println!("{} is {}", cmd_name, found_path.display());
            } else {
                invalid_command(cmd_name);
            }
        }
    }
}

fn find_in_path(cmd_name: &str) -> Option<PathBuf> {
    let path_var = env::var("PATH").ok()?;
    for dir in path_var.split(':') {
        let mut candidate = PathBuf::from(dir);
        candidate.push(cmd_name);
        if candidate.is_file() && is_executable(&candidate) {
            return Some(candidate);
        }
    }
    None
}

fn handle_echo(args: &[&str]) {
    println!("{}", args.join(" "));
}

fn handle_exit() -> ExitCode {
    // println!("Exiting shell...");
    ExitCode::SUCCESS
}
