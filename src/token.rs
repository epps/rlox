#![allow(dead_code)]

#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens:
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One-or-two character tokens:
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals:
    Identifier,
    String,
    Number,
    // Keywords:
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nul,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub text: String,
    /**
    In [Recognizing Lexemes](https://craftinginterpreters.com/scanning.html#recognizing-lexemes), the
    Java type `Object` is used for `literal` given a literal could be one of any of the types in the
    Java type system and `Object` is the "root type" of all of them.

    In Rust, it looks like options include:
    * Enums
    * Traits
    * Generics

    See [here](https://stackoverflow.com/questions/60989833/rust-generic-vectorvectort) and/or
    [here](https://stackoverflow.com/questions/66087633/how-to-create-vec-of-references-to-generic-trait-objects-in-rust)
    for possible reconciliation options.

    Given that it's unclear how the literals will be used and the need to evaluate the PROs and CONs of the
    different approaches in Rust, I'm opting to leave the literal as an unparsed string.
    */
    pub literal: String,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, text: String, literal: String, line: usize) -> Token {
        Token {
            token_type,
            text,
            literal,
            line,
        }
    }
}
