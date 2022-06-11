use std::env;
use std::process;

mod lox;

// See https://www.freebsd.org/cgi/man.cgi?query=sysexits for details
const EX_USAGE: i32 = 64;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() > 1 {
        eprintln!("Usage: rlox [script]");
        process::exit(EX_USAGE);
    } else if args.len() == 1 {
        lox::run_file(&args[0]);
    } else {
        lox::run_prompt();
    }
}
