mod core;
mod errors;
mod lexer;
mod parser;
mod validation;
use std::env;

use calculator::run;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if let Err(e) = run(args) {
        eprintln!("Application Error ~~~~~~~~ {}", e);
        std::process::exit(1);
    }
}
