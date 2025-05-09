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
        }
        println!("{}: command not found", input.trim());
    }
}
