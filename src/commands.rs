// SHELL COMMANDS

use std::env::current_dir;
use std::io::{stdin, BufReader, Read, Write};
use std::{env, fs::File, path::Path, str::SplitWhitespace};

pub fn cd(args: SplitWhitespace) {
    let new_dir = args.peekable().peek().map_or("/", |x| *x);
    let root = Path::new(new_dir);

    if let Err(e) = env::set_current_dir(&root) {
        eprintln!("{}", e);
    }
}

pub fn clear() {
    match term_size::dimensions() {
        Some((_, h)) => {
            for _i in 0..h {
                println!();
            }
        }
        None => {
            println!("Unable to get the window size!");
        }
    }
}

pub fn cat(args: SplitWhitespace) {
    for arg in args {
        match arg {
            arg if arg.starts_with(">") => {
                let path = arg.trim_matches('>');
                let mut file = match File::create(path) {
                    Ok(file) => file,
                    Err(e) => {
                        eprintln!("{e}");
                        return;
                    }
                };

                let mut input = String::new();
                stdin().read_line(&mut input).unwrap();

                match file.write_all(input.as_bytes()) {
                    Ok(_) => {
                        return;
                    }
                    Err(e) => {
                        eprintln!("{e}");
                    }
                };
            }
            _ => {
                let file = match File::open(arg) {
                    Ok(file) => file,
                    Err(_) => {
                        eprintln!("cat: {}: No such file or directory", arg);
                        return;
                    }
                };
                let mut buf_reader = BufReader::new(file);
                let mut contents: String = String::new();
                buf_reader.read_to_string(&mut contents).unwrap();

                println!("{contents}");
            }
        }
    }
}

pub fn pwd() {
    match current_dir() {
        Ok(dir) => {
            println!("{}", dir.display());
        }
        Err(e) => {
            eprintln!("{e}");
        }
    }
}
