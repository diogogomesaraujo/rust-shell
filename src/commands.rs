// SHELL COMMANDS

use crate::color;
use std::env::current_dir;
use std::io::{stdin, BufReader, Read, Write};
use std::{env, fs, fs::File, path::Path, str::SplitWhitespace};

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

pub fn mkdir(args: SplitWhitespace) {
    for arg in args {
        match fs::create_dir(arg) {
            Ok(create) => create,
            Err(e) => {
                eprintln!("{e}");
            }
        };
        break;
    }
}

pub fn grep(args: SplitWhitespace) {
    let mut path: String = String::new();
    let mut word: String = String::new();
    let mut flag: String = String::new();
    let mut output: Vec<String> = Vec::new();

    for arg in args {
        match arg {
            "-i" | "-v" | "-n" | "-w" | "-c" | "--ignore-case" | "--invert-match"
            | "--line-number" => {
                flag = String::from(arg);
            }
            _ if arg.starts_with('"') => {
                word = String::from(arg);
                word = String::from(word.trim_matches('"'));
            }
            _ if arg.starts_with('-') => {
                eprintln!("Invalid flag. The valid ones are -i, -v,-n, -w, or -c.");
                return;
            }
            _ => {
                path = String::from(arg);
            }
        }
    }

    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };
    let mut buf_reader = BufReader::new(file);
    let mut contents: String = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    let highlight_word = |word: &String, line: String| {
        let new_line = line.replace(word, &color::red_text(word.clone()));
        new_line
    };

    match flag.as_str() {
        "-i" | "--ignore-case" => {
            println!("Not yet implemented.");
            return;
        }
        "-v" | "--invert-match" => {
            let lines = contents.split("\n");
            let aux = word.clone();
            for line in lines {
                if !line.contains(aux.as_str()) {
                    output.push(String::from(line));
                }
            }
        }
        "-n" | "--line-number" => {
            let lines = contents.split("\n");
            let aux = word.clone();
            let mut i = 1;
            for line in lines {
                if line.contains(aux.as_str()) {
                    let mut new_line = highlight_word(&word, String::from(line));
                    let line_number = color::teal_text(format!("{i}: "));
                    new_line = format!("{}{}", line_number, new_line);
                    output.push(new_line);
                }
                i += 1;
            }
        }
        "-w" => {
            let lines = contents.split("\n");
            let aux = word.clone();
            for line in lines {
                for w in line.split_whitespace() {
                    if line.contains(aux.as_str()) && word == w {
                        let new_line = highlight_word(&word, String::from(line));
                        output.push(new_line);
                    }
                }
            }
        }
        "-c" => {
            let mut count = 0;
            let lines = contents.split("\n");
            let aux = word.clone();
            for line in lines {
                if line.contains(aux.as_str()) {
                    count += 1;
                }
            }

            println!("{count}");
            return;
        }
        _ => {
            let lines = contents.split("\n");
            let aux = word.clone();
            for line in lines {
                if line.contains(aux.as_str()) {
                    let new_line = highlight_word(&word, String::from(line));
                    output.push(new_line);
                }
            }
        }
    }

    for line in output {
        println!("{line}");
    }
}
