#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::ExitCode;

fn main() -> ExitCode {
    let stdin = io::stdin();
    loop {
        let mut input = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();
        stdin.read_line(&mut input).unwrap();
        if input.trim() == "exit 0" {
            return ExitCode::SUCCESS;
        } else if input.trim().starts_with("echo") {
            println!("{}", input.strip_prefix("echo").unwrap().trim())
        } else {
            println!("{}: command not found", input.trim());
        }
    }
}
