// SHELL COMMANDS

use std::{env, path::Path, str::SplitWhitespace};

pub fn cd(args: SplitWhitespace<>) {
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
        },
        None => {
            println!("Unable to get the window size!");
        }
    }
}

pub fn cat(args: SplitWhitespace<>) {
    
}