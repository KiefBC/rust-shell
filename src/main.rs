#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {
    let stdin = io::stdin();
    loop {
        let mut input = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();
        stdin.read_line(&mut input).unwrap();
        if input.trim() == "exit 0" {
            process::exit(0x0100);
        }
        println!("{}: command not found", input.trim());
    }
}
