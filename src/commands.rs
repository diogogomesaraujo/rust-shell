// SHELL COMMANDS

use crate::color;
use std::env::current_dir;
use std::io::{stdin, stdout, BufReader, Read, Write};
use std::{env, fs, fs::File, path::Path};

pub fn ls(/*args: Vec<String>*/) -> Option<String> {
    let paths = match fs::read_dir("./") {
        Ok(paths) => paths,
        Err(e) => {
            eprintln!("{e}");
            return None;
        }
    };

    let mut result = String::new();

    /* for arg in args {
        // to implement
        match arg.as_str() {
            "-l" => {}
            "-a" => {}
            "-t" => {}
            "-r" => {}
            "-S" => {}
            "-R" => {}
            "-i" => {}
            "-g" => {}
            "-h" => {}
            "-d" => {}
            _ => {}
        }
    } */

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

pub fn echo(args: Vec<String>) -> Option<String> {
    if let Some(flag) = args.get(0) {
        println!("{}", flag);
        match flag.as_str() {
            "-e" => {
                if args[1].starts_with('"') && args[args.len() - 1].ends_with('"') {
                    println!();
                    let mut contents = String::new();
                    for arg in args {
                        match arg.as_str() {
                            "\n" => {
                                println!();
                                contents.push('\n');
                            }
                            "\t" => {
                                print!("    ");
                                stdout().flush().unwrap();
                                contents.push('\t');
                            }
                            _ => {
                                let out = format!("{} ", arg);
                                print!("{}", &out);
                                stdout().flush().unwrap();
                                contents.push_str(out.as_str());
                            }
                        }
                    }
                }
            }
            _ => {
                if args[0].starts_with('"') && args[args.len() - 1].ends_with('"') {
                    let contents = args.join(" ");
                    println! {"{}", &contents};
                    return Some(contents.to_string());
                }
            }
        }
    }
    None
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
    let mut paths: Vec<String> = Vec::new();
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
                continue;
            }
            _ => {
                paths.push(arg);
            }
        }
    }

    for path in paths {
        let file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("{e}");
                continue;
            }
        };

        let mut buf_reader = BufReader::new(file);
        let mut contents: String = String::new();
        match buf_reader.read_to_string(&mut contents) {
            Ok(ok) => ok,
            Err(_) => {
                continue;
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
                let aux = &word;
                for line in lines {
                    if !line.contains(aux.as_str()) {
                        output.push(String::from(line));
                    }
                }
            }
            "-n" | "--line-number" => {
                let lines = contents.split("\n");
                let aux = &word;
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
                let aux = &word;
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
                let aux = &word;
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
                let aux = &word;
                for line in lines {
                    if line.contains(aux.as_str()) {
                        let new_line = highlight_word(&word, String::from(line));
                        output.push(new_line);
                    }
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

pub fn du(args: Vec<String>) -> Option<String> {
    if args.len() != 1 {
        eprintln!("Invalid number of arguments");
        return None;
    }
    let path = &args[0];
    match File::open(&path) {
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
            let size_of_file = contents.as_bytes().len();

            let mut result: String = String::from("Size of file is ");
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

pub fn head(args: Vec<String>) -> Option<String> {
    let mut result: String = String::new();
    let mut flag: String = String::new();
    let mut paths: Vec<String> = Vec::new();
    let mut n: u32 = 10;

    for arg in args {
        match arg.as_str() {
            _ if arg.starts_with("-") => {
                flag = arg;
            }
            _ => match arg.parse::<u32>() {
                Ok(parsed_arg) => {
                    n = parsed_arg;
                }
                Err(_) => {
                    paths.push(arg);
                }
            },
        }
    }

    match flag.as_str() {
        "-n" | "" => {
            for path in paths {
                let mut header: String = String::from("==> ");
                header.push_str(path.as_str());
                header.push_str(" <==");
                header = color::teal_text(header);
                println!("{}", &header);
                result.push_str(header.as_str());
                result.push('\n');
                match File::open(path) {
                    Ok(file) => {
                        let mut buf_reader = BufReader::new(file);
                        let mut contents: String = String::new();
                        match buf_reader.read_to_string(&mut contents) {
                            Ok(ok) => ok,
                            Err(e) => {
                                eprintln!("{}", e);
                                continue;
                            }
                        };

                        let mut aux = 0;
                        for line in contents.split('\n') {
                            if aux == n {
                                break;
                            }
                            println!("{}", &line);
                            result.push_str(line);
                            aux += 1;
                        }
                    }
                    Err(e) => {
                        println!("{e}");
                    }
                }
            }
        }
        "-c" => {
            for path in paths {
                let mut header: String = String::from("==> ");
                header.push_str(path.as_str());
                header.push_str(" <==");
                header = color::teal_text(header);
                println!("{}", &header);
                result.push_str(header.as_str());
                result.push('\n');
                match File::open(path) {
                    Ok(file) => {
                        let mut buf_reader = BufReader::new(file);
                        let mut contents: String = String::new();
                        match buf_reader.read_to_string(&mut contents) {
                            Ok(ok) => ok,
                            Err(e) => {
                                eprintln!("{}", e);
                                continue;
                            }
                        };

                        let mut aux = 0;

                        for ch in contents.chars() {
                            if aux == n {
                                break;
                            }
                            print!("{}", &ch);
                            stdout().flush().unwrap();
                            result.push(ch);
                            aux += 1;
                        }
                        println!();
                        result.push_str("\n");
                    }
                    Err(e) => {
                        println!("{e}");
                    }
                }
            }
        }
        "-v" => {
            for path in paths {
                match File::open(path) {
                    Ok(file) => {
                        let mut buf_reader = BufReader::new(file);
                        let mut contents: String = String::new();
                        match buf_reader.read_to_string(&mut contents) {
                            Ok(ok) => ok,
                            Err(e) => {
                                eprintln!("{}", e);
                                continue;
                            }
                        };

                        let mut aux = 0;
                        for line in contents.split('\n') {
                            if aux == n {
                                break;
                            }
                            println!("{}", &line);
                            result.push_str(line);
                            aux += 1;
                        }
                    }
                    Err(e) => {
                        println!("{e}");
                    }
                }
            }
        }
        _ => {}
    }

    return Some(result);
}

pub fn wc(args: Vec<String>) -> Option<String> {
    let mut flag: String = String::new();
    let mut files: Vec<String> = Vec::new();
    let mut result: String = String::new();

    for arg in args {
        match arg {
            f if f.starts_with('-') => {
                flag = f;
            }
            _ => {
                files.push(arg);
            }
        }
    }

    for file in files {
        match File::open(&file) {
            Ok(f) => {
                let mut buf_reader = BufReader::new(f);
                let mut contents: String = String::new();
                match buf_reader.read_to_string(&mut contents) {
                    Ok(_) => {
                        let lines = contents.split("\n").count();
                        let words = contents.split_whitespace().count();
                        let bytes = contents.as_bytes().len();

                        match flag.as_str() {
                            "-l" => {
                                let line = format!("{} {}\n", lines, file);
                                println!("{}", line.trim());
                                result.push_str(line.as_str());
                            }
                            "-c" => {
                                let line = format!("{} {}\n", bytes, file);
                                println!("{}", line.trim());
                                result.push_str(line.as_str());
                            }
                            "-w" => {
                                let line = format!("{} {}\n", words, file);
                                println!("{}", line.trim());
                                result.push_str(line.as_str());
                            }
                            _ => {
                                let line = format!("{} {} {} {}\n", lines, words, bytes, file);
                                println!("{}", line.trim());
                                result.push_str(line.as_str());
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                };
            }
            Err(e) => {
                eprintln!("{e}");
            }
        }
    }

    Some(result)
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

pub fn cp(args: Vec<String>) -> Option<String> {
    if args.len() < 2 {
        eprintln!("not enough arguments!");
        return None;
    }
    let copied_files: &Vec<String> = &args[..(args.len() - 1)].to_vec();
    let destiny_directory = &args[args.len() - 1];

    for file_path in copied_files {
        match File::open(file_path) {
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
                match fs::read_dir(destiny_directory) {
                    Ok(_) => {
                        let file_path = match file_path.split('/').rev().next() {
                            Some(path) => path,
                            None => file_path,
                        };
                        let file_path = format!("{}/{}", destiny_directory, file_path);

                        match File::create(file_path) {
                            Ok(mut file) => match File::write(&mut file, contents.as_bytes()) {
                                Ok(_) => (),
                                Err(e) => {
                                    eprintln!("{e}");
                                    return None;
                                }
                            },
                            Err(e) => {
                                eprintln!("{e}");
                                return None;
                            }
                        }
                    }
                    Err(_) => {
                        let mut aux_dir = String::new();

                        for dir in destiny_directory.split('/') {
                            aux_dir = match aux_dir.as_str() {
                                "" => dir.to_string(),
                                _ => format!("{}/{}", aux_dir, dir),
                            };

                            fs::create_dir(aux_dir.clone()).expect("should not happen");
                        }

                        let file_path = match file_path.split('/').rev().next() {
                            Some(path) => path,
                            None => file_path,
                        };
                        let file_path = format!("{}/{}", aux_dir, file_path);

                        match File::create(file_path) {
                            Ok(mut file) => match File::write(&mut file, contents.as_bytes()) {
                                Ok(_) => (),
                                Err(e) => {
                                    eprintln!("{e}");
                                    return None;
                                }
                            },
                            Err(e) => {
                                eprintln!("{e}");
                                return None;
                            }
                        };
                    }
                };
            }
            Err(e) => {
                eprintln!("{e}");
            }
        }
    }
    None
}
