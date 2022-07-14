#![allow(dead_code)]
use crate::token::Token;

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
    Literal(LiteralType),
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

pub enum LiteralType {
    // TODO: Look into improving this.
    Number(f64),
    String(String),
    True(bool),
    False(bool),
    Nil(()),
}
