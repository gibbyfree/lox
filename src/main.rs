use std::env;
use std::fs;
use std::io::Error;
use std::io::{self, BufRead};

use backend::scanner::Scanner;
use data::token::Token;

mod backend;
mod data;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        panic!("Usage: loxrs [script]");
    } else if std::env::args().count() == 2 {
        run_file(&args[0]);
    } else {
        run_prompt();
    }
}

fn run_file(path: &String) {
    let src = fs::read_to_string(path).expect("Unable to read file at the given path.");
    run(src).unwrap_or_else(|err| {
        eprintln!("omg!!! {}", err);
    })
}

fn run_prompt() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(line) => run(line).unwrap_or_else(|err| {
                eprintln!("omg!!! {}", err);
            }),
            Err(_e) => break,
        }
    }
}

fn run(source: String) -> Result<(), Error> {
    let scanner: Scanner = Scanner::new(source);
    let tokens: Vec<Token> = scanner.scan_tokens();
    for token in tokens {
        println!("{:?}", token);
    }
    Ok(())
}

// error handling methods

fn error(line: i16, msg: String) {
    report(line, "".to_string(), msg);
}

fn report(line: i16, location: String, msg: String) {
    eprintln!("[line {}] Error {}: {}", line, location, msg);
}
