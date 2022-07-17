use std::env;

use lox_interpreter_rust::lox;

fn main() {
    let args: Vec<String> = env::args().collect();
    lox::run(&args);
}
