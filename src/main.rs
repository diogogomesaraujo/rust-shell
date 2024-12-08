use chrono::prelude::*;
use std::env::current_dir;
use std::io::Write;
use std::process::exit;

mod color;
mod commands;
mod custom_io;

fn main() {
    commands::clear();
    commands::init();
    let prefix: String = color::red_text(String::from("diogo"));
    let mut commands: Vec<String> = Vec::new();
    let mut _previous_command: Option<String> = None;

    loop {
        let current_dir = current_dir().unwrap();
        let time = Local::now().format("%H:%M%P");
        let mut previous_output: Option<String> = None;

        println!(
            "[ {0} {1} ]: {2}",
            prefix,
            time,
            color::teal_text(current_dir.display().to_string())
        );

        print!("{}", color::teal_text(String::from("$ ")));
        std::io::stdout().flush().unwrap();

        let input = custom_io::read_instance(&mut commands);

        let mut commands_from_input = input.split(" | ").peekable();

        while let Some(command_input) = commands_from_input.next() {
            let mut parts = command_input.trim().split_whitespace();
            let command = match parts.next() {
                Some(command) => command,
                None => {
                    continue;
                }
            };
            let args = parts;
            let mut args: Vec<String> = args.map(|s| s.to_string()).collect();
            if let Some(previous_output) = previous_output {
                let previous_output_args: Vec<String> = previous_output
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect();
                for arg in previous_output_args {
                    args.push(arg);
                }
            }

            let output = match command {
                "ls" => commands::ls(args),
                "cd" => commands::cd(args),
                "clear" => commands::clear(),
                "cat" => commands::cat(args),
                "pwd" => commands::pwd(),
                "head" => commands::head(args),
                "mkdir" => commands::mkdir(args),
                "grep" => commands::grep(args),
                "used" => commands::used(args),
                "hashkitten" => commands::hashkitten(args),
                "hi" => commands::init(),
                "exit" => {
                    exit(0);
                }
                _ => {
                    eprintln!("No command found!");
                    break;
                }
            };

            previous_output = output;
        }
        //println!("{:?}", commands);
    }
}
