use std::{io::stdin, process::Command};
use std::env::current_dir;
use chrono::prelude::*;
use std::io::Write;

mod commands;

fn red_text(s: String) -> String {
    let mut aux: String = String::from("\u{001b}[31m");
    aux.push_str(&s);
    aux.push_str("\u{001b}[0m");

    aux
}

fn teal_text(s: String) -> String {
    let mut aux: String = String::from("\u{001b}[36m");
    aux.push_str(&s);
    aux.push_str("\u{001b}[0m");

    aux
}

fn main() {
    commands::clear();
    let prefix: String = red_text(String::from("diogo"));

    loop {
        let current_dir = current_dir().unwrap();
        let time = Local::now().format("%H:%M%P");

        println!("[ {0} {1} ]: {2}", prefix, time, teal_text(current_dir.display().to_string()));

        print!("{}", teal_text(String::from("$ ")));
        std::io::stdout().flush().unwrap();
        
        let mut input: String = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match (command, &args) {
            ("cd", _) => { commands::cd(args); }
            ("clear", _) => { commands::clear(); },
            ("cat", _) => { commands::cat(args); }
            (_, _) => {
                let child = Command::new(command).args(args).spawn();

                match child {
                    Ok(mut child) => { child.wait().unwrap(); },
                    Err(e) => { println!("{e}"); }
                }
            }
        }
    }
}