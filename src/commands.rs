// SHELL COMMANDS

use crate::color;
use std::env::current_dir;
use std::io::{stdin, stdout, BufReader, Read, Write};
use std::{env, fs, fs::File, path::Path};

pub fn ls(args: Vec<String>) -> Option<String> {
    let paths = match fs::read_dir("./") {
        Ok(paths) => paths,
        Err(e) => {
            eprintln!("{e}");
            return None;
        }
    };

    let mut result = String::new();

    for path in paths {
        if let Some(path) = path.unwrap().path().as_os_str().to_str() {
            print!("{}  ", &path[2..]);
            stdout().flush().unwrap();
            result.push_str(&path[2..]);
            result.push_str(" ");
        }
    }
    println!();
    return Some(result);
}

pub fn cd(args: Vec<String>) -> Option<String> {
    if args.len() != 1 {
        eprintln!("Invalid number of arguments");
        return None;
    }
    let root = Path::new(&args[0]);

    if let Err(e) = env::set_current_dir(&root) {
        eprintln!("{}", &e);
        return None;
    }
    None
}

pub fn clear() -> Option<String> {
    match term_size::dimensions() {
        Some((_, h)) => {
            for _i in 0..h {
                println!();
            }

            return None;
        }
        None => {
            println!("Unable to get the window size!");
            return None;
        }
    }
}

pub fn cat(args: Vec<String>) -> Option<String> {
    let mut result: String = String::new();
    for arg in args {
        match arg {
            arg if arg.starts_with(">") => {
                let path = arg.trim_matches('>');
                let mut file = match File::create(path) {
                    Ok(file) => file,
                    Err(e) => {
                        eprintln!("{}", e);
                        continue;
                    }
                };

                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(ok) => ok,
                    Err(e) => {
                        eprintln!("{e}");
                        continue;
                    }
                };

                match file.write_all(input.as_bytes()) {
                    Ok(_) => {
                        continue;
                    }
                    Err(e) => {
                        eprintln!("{e}");
                        continue;
                    }
                };
            }
            _ => {
                let file = match File::open(&arg) {
                    Ok(file) => file,
                    Err(_) => {
                        eprintln!("cat: {}: No such file or directory", arg);
                        continue;
                    }
                };
                let mut buf_reader = BufReader::new(file);
                let mut contents: String = String::new();
                match buf_reader.read_to_string(&mut contents) {
                    Ok(ok) => ok,
                    Err(e) => {
                        eprintln!("{e}");
                        continue;
                    }
                };

                println!("{}", &contents);
                result.push_str(contents.as_str());
            }
        }
    }
    return Some(result);
}

pub fn pwd() -> Option<String> {
    match current_dir() {
        Ok(dir) => {
            println!("{}", dir.display());
            return Some(String::from(dir.to_str().unwrap()));
        }
        Err(e) => {
            eprintln!("{e}");
            return None;
        }
    }
}

pub fn mkdir(args: Vec<String>) -> Option<String> {
    for arg in args {
        match fs::create_dir(arg) {
            Ok(create) => create,
            Err(e) => {
                eprintln!("{e}");
            }
        };
        break;
    }
    return None;
}

pub fn grep(args: Vec<String>) -> Option<String> {
    // needs to restructure to handle mutiple files
    let mut path: String = String::new();
    let mut word: String = String::new();
    let mut flag: String = String::new();
    let mut output: Vec<String> = Vec::new();

    for arg in args {
        match arg.as_str() {
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
                return None;
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
            return None;
        }
    };
    let mut buf_reader = BufReader::new(file);
    let mut contents: String = String::new();
    match buf_reader.read_to_string(&mut contents) {
        Ok(ok) => ok,
        Err(e) => {
            eprintln!("{e}");
            return None;
        }
    };

    let highlight_word = |word: &String, line: String| {
        let new_line = line.replace(word, &color::red_text(word.clone()));
        new_line
    };

    match flag.as_str() {
        "-i" | "--ignore-case" => {
            println!("Not yet implemented.");
            return None;
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

            let output: String = count.to_string();
            println!("{}", &output);
            return Some(output);
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
    let mut result: String = String::new();
    for line in &output {
        println!("{}", &line);
        result.push_str(line);
    }
    return Some(result);
}

pub fn used(args: Vec<String>) -> Option<String> {
    if args.len() != 1 {
        eprintln!("Invalid number of arguments");
        return None;
    }
    let path = &args[0];
    match File::open(path) {
        Ok(file) => {
            let mut buf_reader = BufReader::new(file);
            let mut contents: String = String::new();
            match buf_reader.read_to_string(&mut contents) {
                Ok(ok) => ok,
                Err(e) => {
                    eprintln!("{}", e);
                    return None;
                }
            };
            let size_of_file = contents.len();

            let mut result: String = String::from("Size of fiel is ");
            let aux = size_of_file as f64 / (1024 * 1024) as f64;
            result.push_str(size_of_file.to_string().as_str());
            result.push_str(" bytes (");
            result.push_str(aux.to_string().as_str());
            result.push_str(" MB)");

            println!("{}", &result);

            return Some(result);
        }
        Err(_) => {
            return None;
        }
    }
}

pub fn init() -> Option<String> {
    let file = match File::open("hi_there") {
        Ok(file) => file,
        Err(_) => {
            return None;
        }
    };

    let mut buf_reader = BufReader::new(file);
    let mut contents: String = String::new();
    match buf_reader.read_to_string(&mut contents) {
        Ok(ok) => ok,
        Err(e) => {
            eprintln!("{}", e);
            return None;
        }
    };

    println!("{}", contents);
    let size_of_contents = contents.split('\n').count();
    match term_size::dimensions() {
        Some((_, h)) => {
            if size_of_contents + 2 < h {
                for _i in 0..(h - size_of_contents - 2) {
                    println!();
                }
            }
        }
        None => {}
    };
    return Some(contents);
}

pub fn hashkitten(args: Vec<String>) -> Option<String> {
    let mut args = args;
    args.push(String::from("hashkitten"));
    args.reverse();
    hashkitten::run(args);
    return None;
}
