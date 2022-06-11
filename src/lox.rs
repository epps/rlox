use std::fs;
use std::io::{self, Write};

pub fn run_file(path: &String) {
    let contents = fs::read_to_string(path).expect("Failed to read file");
    run(contents);
}

pub fn run_prompt() {
    let mut line = String::new();

    loop {
        print!("> ");
        // https://stackoverflow.com/a/54263074
        let _ = io::stdout().flush();
        // https://doc.rust-lang.org/std/io/trait.BufRead.html#method.read_line
        let num_bytes = io::stdin()
            .read_line(&mut line)
            .expect("Reading from stdin won't fail");

        match num_bytes {
            0 => break,
            _ => run(line.clone()),
        };

        line.clear();
    }
}

fn run(source: String) {
    let scanner = Scanner::new(source);
    let tokens = scanner.scanTokens();

    for token in tokens {
        println!("{token}");
    }
}

struct Scanner {
    source: String,
}

impl Scanner {
    fn new(source: String) -> Scanner {
        Scanner { source }
    }

    fn scanTokens(&self) -> Vec<Token> {
        self.source.split(" ").collect()
    }
}

type Token<'a> = &'a str;
