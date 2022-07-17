use std::fs;
use std::io;
use std::io::Write;

use crate::lexer::Lexer;

pub fn run(args: &Vec<String>) {
    if args.len() > 2 {
        println!("Usage: jlox [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(path: &String) {
    let source = fs::read_to_string(path).expect("Read file from path");
    run_lox(source);
}

fn run_prompt() {
    print_repl_welcome_message();
    repl();
}

fn repl() {
    loop {
        print!("> ");
        // Flush stdout so it prints immediately.
        io::stdout().flush().expect("Read input from REPL");

        let mut input_buffer = String::new();

        match io::stdin().read_line(&mut input_buffer) {
            Ok(_) => {
                run_lox(input_buffer);
            }
            _ => break,
        }
    }
}

fn print_repl_welcome_message() {
    println!("");
    println!("Welcome to Lox REPL written in Rust!");
    println!("Press Ctrl-C to exit.");
    println!("");
}

fn run_lox(source: String) {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();
    let lexer_errors = lexer.errors;

    for token in tokens {
        println!("{token}");
    }

    for error in lexer_errors {
        println!("{error}");
    }
}
