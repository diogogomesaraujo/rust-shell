use chrono::prelude::*;
use std::env::current_dir;
use std::io::Write;
use std::process::Command;

mod color;
mod commands;
mod custom_io;

fn main() {
    commands::clear();
    let prefix: String = color::red_text(String::from("diogo"));

    loop {
        let current_dir = current_dir().unwrap();
        let time = Local::now().format("%H:%M%P");

        println!(
            "[ {0} {1} ]: {2}",
            prefix,
            time,
            color::teal_text(current_dir.display().to_string())
        );

        print!("{}", color::teal_text(String::from("$ ")));
        std::io::stdout().flush().unwrap();

        let input = custom_io::read_instance();

        let mut parts = input.trim().split_whitespace();
        let command = match parts.next() {
            Some(command) => command,
            None => {
                continue;
            }
        };
        let args = parts;

        match (command, &args) {
            ("cd", _) => {
                commands::cd(args);
            }
            ("clear", _) => {
                commands::clear();
            }
            ("cat", _) => {
                commands::cat(args);
            }
            ("pwd", _) => {
                commands::pwd();
            }
            (_, _) => {
                let child = Command::new(command).args(args).spawn();

                match child {
                    Ok(mut child) => {
                        child.wait().unwrap();
                    }
                    Err(e) => {
                        println!("{e}");
                    }
                }
            }
        }
    }
}
