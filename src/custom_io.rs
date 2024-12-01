use console::Term;
use std::io::{stdout, Write};

pub fn read_instance() -> String {
    let mut input = String::new();
    let mut cursor_position: usize = 0;
    loop {
        let mut term = Term::stdout();
        match term.read_key() {
            Ok(key) => match key {
                console::Key::ArrowLeft => {
                    if cursor_position != 0 {
                        let _result = term.write_all(b"\x1b[1D");
                        cursor_position -= 1;
                    }
                }
                console::Key::ArrowRight => {
                    if cursor_position < input.len() {
                        let _result = term.write_all(b"\x1b[1C");
                        cursor_position += 1;
                    }
                }
                console::Key::Enter => {
                    println!();
                    return input;
                }
                console::Key::Backspace => {
                    if !input.is_empty() {
                        let moves = input.len() - cursor_position;
                        if moves > 0 {
                            term.write_all(format!("\x1b[{}C", moves).as_bytes())
                                .unwrap();
                        }

                        input.pop();
                        term.clear_chars(1).unwrap();
                        cursor_position = input.len();
                    }
                }
                console::Key::Char(c) => {
                    input.insert(cursor_position, c);
                    cursor_position += 1;
                    if cursor_position != input.len() {
                        input.remove(cursor_position);
                    }
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
