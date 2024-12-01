use std::io::{stdin, stdout, Read, Write};

use console::Term;

pub fn read_instance() -> String {
    let mut input = String::new();
    let mut cursor_position: usize = 0;
    loop {
        let mut term = Term::stdout();
        match term.read_key() {
            Ok(key) => match key {
                console::Key::ArrowLeft => {
                    term.write_all(b"\x1b[1D");
                }
                console::Key::ArrowRight => {
                    if cursor_position + 1 < input.len() {
                        term.write_all(b"\x1b[1C");
                        cursor_position += 1;
                    }
                }
                console::Key::Enter => {
                    println!();
                    return input;
                }
                console::Key::Backspace => {
                    if !input.is_empty() {
                        input.pop();
                        term.clear_chars(1).unwrap();
                    }
                }
                console::Key::Char(c) => {
                    input.push(c);
                    print!("{c}");
                    stdout().flush().unwrap();
                }
                _ => {
                    continue;
                }
            },
            Err(_) => {}
        };
    }
}
