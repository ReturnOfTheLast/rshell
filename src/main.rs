use std::io::{stdin, stdout, Write};
use std::process::Command;
use std::path::Path;
use std::env;

fn main() {
    loop {
        // use the `>` character as the prompt
        // need to explicitly flush this to ensure it prints before read_line
        print!("> ");
        stdout().flush().unwrap();

        // get input
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // split the input into multiple parts for the args
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        // Check if inbuilt command
        match command {
            "cd" => {
                // Default to '/' as new directory if one was not provided
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e)
                }
            },
            "exit" => return,
            command => {
                // spawn process
                let child = Command::new(command)
                    .args(args)
                    .spawn();

                // graceful error handling
                match child {
                    Ok(mut child) => { child.wait().unwrap(); },
                    Err(e) => eprintln!("{}", e),
                }
            }
        }
    }
}
