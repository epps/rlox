use std::{iter::Peekable, str::Chars};

use crate::errors;
use crate::token::{Token, TokenType};

pub struct Scanner<'a> {
    source: &'a str,
    source_iter: Peekable<Chars<'a>>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner<'_> {
    // Equivalent lifetime syntax:
    // pub fn new<'a>(source: &'a str) -> Scanner<'a> {
    // See Lifetime Elision: https://doc.rust-lang.org/nomicon/lifetime-elision.html
    pub fn new(source: &str) -> Scanner<'_> {
        Scanner {
            source,
            source_iter: source.chars().peekable(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            String::new(),
            String::new(),
            self.line,
        ));

        &self.tokens
    }

    fn scan_token(&mut self) {
        let char = self.advance().unwrap();
        match char {
            '(' => self.add_token(TokenType::LeftParen, String::new()),
            ')' => self.add_token(TokenType::RightParen, String::new()),
            '{' => self.add_token(TokenType::LeftBrace, String::new()),
            '}' => self.add_token(TokenType::RightBrace, String::new()),
            ',' => self.add_token(TokenType::Comma, String::new()),
            '.' => self.add_token(TokenType::Dot, String::new()),
            '-' => self.add_token(TokenType::Minus, String::new()),
            '+' => self.add_token(TokenType::Plus, String::new()),
            ';' => self.add_token(TokenType::Semicolon, String::new()),
            '*' => self.add_token(TokenType::Star, String::new()),
            '!' => {
                if self.is_match('=') {
                    self.add_token(TokenType::BangEqual, String::new());
                } else {
                    self.add_token(TokenType::Bang, String::new());
                }
            }
            '=' => {
                if self.is_match('=') {
                    self.add_token(TokenType::EqualEqual, String::new());
                } else {
                    self.add_token(TokenType::Equal, String::new());
                }
            }
            '<' => {
                if self.is_match('=') {
                    self.add_token(TokenType::LessEqual, String::new());
                } else {
                    self.add_token(TokenType::Less, String::new());
                }
            }
            '>' => {
                if self.is_match('=') {
                    self.add_token(TokenType::GreaterEqual, String::new())
                } else {
                    self.add_token(TokenType::Greater, String::new());
                }
            }
            '/' => {
                if self.is_match('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, String::new());
                }
            }
            ' ' => {}
            '\r' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => self.string(),
            c => {
                if self.is_digit(c) {
                    self.number(c);
                } else {
                    errors::handle(self.line, format!("Unrecognized token: {}", char));
                }
            }
        }
    }

    fn number(&mut self, c: char) {
        let mut number_chars = vec![c];

        let next = self.peek();

        while self.is_digit(next) {
            number_chars.push(self.advance().unwrap());
        }

        /*
        [4.6.2 Number literals](https://craftinginterpreters.com/scanning.html#number-literals)
        introduces a `peekNext` method to look ahead
        by 2 indices; however, this lookahead is complicated using the peekable
        iterator. For now, I'll try to handle this case differently; however,
        if `peekNext` is used in the future in a way that I can't account for with
        "manual" logic, I may have to look for
        [an alternative](https://stackoverflow.com/questions/62186871/how-to-correctly-use-peek-in-rust).
         */
        if next == '.' {
            let fraction_char = self.advance().unwrap();
            let next = self.peek();
            if self.is_digit(next) {
                number_chars.push(fraction_char);
                loop {
                    let a = self.advance();
                    number_chars.push(a.unwrap());
                    let next = self.peek();
                    if !self.is_digit(next) {
                        break;
                    }
                }
            }
        }

        let value: String = number_chars.into_iter().collect();
        self.add_token(TokenType::Number, value);
    }

    fn string(&mut self) {
        let mut string_chars = vec![];

        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            string_chars.push(self.advance().unwrap());
        }

        if self.is_at_end() {
            errors::handle(self.line, String::from("Unterminated string."));
            return;
        }

        self.advance(); // The closing '""

        let value: String = string_chars.into_iter().collect();
        self.add_token(TokenType::String, value);
    }

    fn is_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if *self.source_iter.peek().unwrap() != expected {
            return false;
        }

        self.advance();
        true
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        *self.source_iter.peek().unwrap()
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> Option<char> {
        let char = self.source_iter.next();
        self.current += 1;
        char
    }

    fn add_token(&mut self, token_type: TokenType, text: String) {
        self.tokens
            .push(Token::new(token_type, text, String::new(), self.line))
    }
}
