use chrono::prelude::*;
use std::env::current_dir;
use std::io::Write;
use std::process::Child;
use std::{io::stdin, process::Command};
use std::process::Stdio;

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
    let prefix: String = red_text(String::from("kitten"));

    loop {
        let current_dir = current_dir().unwrap();
        let time = Local::now().format("%H:%M%P");

        println!(
            "[ {0} {1} ]: {2}",
            prefix,
            time,
            teal_text(current_dir.display().to_string())
        );

        print!("{}", teal_text(String::from("$ ")));
        std::io::stdout().flush().unwrap();

        let mut input: String = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split("|").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match (command, &args) {
                ("cd", _) => {
                    commands::cd(args);
                    previous_command = None;
                },
                ("clear", _) => {
                    commands::clear();
                    previous_command = None;
                },
                ("exit", _) => { return; }
                (_, _) => {
                    let stdin = previous_command.map_or(
                        Stdio::inherit(),
                        |output: Child| Stdio::from(output.stdout.unwrap())
                    );

                    let stdout = if commands.peek().is_some() {
                        // there is another command piped behind this one
                        // prepare to send output to the next command
                        Stdio::piped()
                    } else {
                        // there are no more commands piped behind this one
                        // send output to shell stdout
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => { previous_command = Some(output); },
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        },
                    };
                }
            }
        }
    }
}
