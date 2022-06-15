use std::{iter::Peekable, str::Chars};

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

    fn scan_token(&self) {}

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
