use console::Term;
use std::io::{stdout, Write};

use crate::color::{self};

pub fn read_instance(commands: &mut Vec<String>) -> String {
    let mut input = String::new();
    let mut cursor_position: usize = 0;
    let mut current_command: i32 = commands.len() as i32 - 1;
    let mut last_pressed: console::Key = console::Key::Unknown;

    loop {
        let mut term = Term::stdout();
        match term.read_key() {
            Ok(key) => match key {
                console::Key::ArrowLeft => {
                    if cursor_position != 0 {
                        let _result = term.write_all(b"\x1b[1D");
                        cursor_position -= 1;
                    }
                    last_pressed = console::Key::ArrowLeft;
                }
                console::Key::ArrowRight => {
                    if cursor_position < input.len() {
                        let _result = term.write_all(b"\x1b[1C");
                        cursor_position += 1;
                    }
                    last_pressed = console::Key::ArrowRight;
                }
                console::Key::ArrowUp => {
                    if current_command - 1 >= 0 {
                        current_command -= 1;
                        term.clear_line().unwrap();
                        print!("{}", color::teal_text(String::from("$ ")));
                        stdout().flush().unwrap();
                        input = commands[current_command as usize].clone();
                        print!("{}", &input);
                        stdout().flush().unwrap();
                        cursor_position = input.len();
                    }
                    last_pressed = console::Key::ArrowUp;
                }
                console::Key::ArrowDown => {
                    if current_command + 1 < commands.len() as i32 {
                        current_command += 1;
                        term.clear_line().unwrap();
                        print!("{}", color::teal_text(String::from("$ ")));
                        stdout().flush().unwrap();
                        input = commands[current_command as usize].clone();
                        print!("{}", &input);
                        stdout().flush().unwrap();
                        cursor_position = input.len();
                    }
                    last_pressed = console::Key::ArrowDown;
                }
                console::Key::Enter => {
                    println!();
                    if last_pressed == console::Key::ArrowUp
                        || last_pressed == console::Key::ArrowUp
                    {
                        if let Some(last) = commands.last_mut() {
                            *last = input.clone();
                        }
                    }
                    if let Some(last) = commands.last_mut() {
                        if last != "" {
                            *last = input.clone();
                            commands.push(String::new());
                        }
                    }
                    return input;
                    last_pressed = console::Key::Enter;
                }
                console::Key::Backspace => {
                    if !input.is_empty() {
                        let moves = input.len() - cursor_position;
                        if moves > 0 {
                            term.write_all(format!("\x1b[{}C", moves).as_bytes())
                                .unwrap();
                        }

                        match commands.last_mut() {
                            Some(last) => {
                                last.pop();
                            }
                            None => {}
                        }

                        input.pop();
                        term.clear_chars(1).unwrap();
                        cursor_position = input.len();
                    }
                    last_pressed = console::Key::Backspace;
                }
                console::Key::Char(c) => {
                    input.insert(cursor_position, c);
                    cursor_position += 1;
                    if cursor_position != input.len() {
                        input.remove(cursor_position);
                    }
                    print!("{}", &c);
                    stdout().flush().unwrap();

                    if commands.is_empty() {
                        commands.push(String::new());
                    }
                    if let Some(last) = commands.last_mut() {
                        last.push(c);
                    }
                    last_pressed = console::Key::Char(c);
                }
                console::Key::CtrlC => {
                    return String::from("");
                }
                _ => {
                    continue;
                }
            },
            Err(_) => {}
        };
    }
}
