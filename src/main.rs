use std::env;
use std::process;

mod ast_printer;
mod errors;
mod exits;
mod expr;
mod lox;
mod scanner;
mod token;
mod visitor;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() > 1 {
        eprintln!("Usage: rlox [script]");
        process::exit(exits::EX_USAGE);
    } else if args.len() == 1 {
        lox::run_file(&args[0]);
    } else {
        lox::run_prompt();
    }
}
