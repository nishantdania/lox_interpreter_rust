use std::env;

use lox_interpreter_rust::cli;

fn main() {
    let args: Vec<String> = env::args().collect();
    cli::run(&args);
}
