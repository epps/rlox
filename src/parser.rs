use crate::expr::{Expr, LiteralType};
use crate::token::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens.clone(),
            current: 0,
        }
    }
    // expression     → equality ;
    fn expression(&mut self) -> Box<Expr> {
        self.equality()
    }
    // equality       → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Box<Expr> {
        let mut expr = self.comparison();
        while self.match_type(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().unwrap().clone();
            let right = self.comparison();
            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right,
            })
        }

        expr
    }
    // comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> Box<Expr> {
        let mut expr = self.term();
        while self.match_type(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().unwrap().clone();
            let right = self.term();
            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right,
            });
        }

        expr
    }
    // term           → factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Box<Expr> {
        let mut expr = self.factor();
        while self.match_type(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().unwrap().clone();
            let right = self.factor();
            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right,
            });
        }

        expr
    }
    // factor         → unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> Box<Expr> {
        let mut expr = self.unary();
        while self.match_type(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().unwrap().clone();
            let right = self.unary();
            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right,
            });
        }

        expr
    }
    // unary          → ( "!" | "-" ) unary | primary ;
    fn unary(&mut self) -> Box<Expr> {
        if self.match_type(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().unwrap().clone();
            let right = self.unary();
            return Box::new(Expr::Unary { operator, right });
        }

        self.primary()
    }
    // primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    fn primary(&mut self) -> Box<Expr> {
        if self.match_type(vec![TokenType::False]) {
            return Box::new(Expr::Literal(LiteralType::False(false)));
        }
        if self.match_type(vec![TokenType::True]) {
            return Box::new(Expr::Literal(LiteralType::True(true)));
        }
        if self.match_type(vec![TokenType::Nil]) {
            return Box::new(Expr::Literal(LiteralType::Nil(())));
        }
        if self.match_type(vec![TokenType::Number, TokenType::String]) {
            if let Some(previous) = self.previous() {
                if previous.token_type == TokenType::Number {
                    return Box::new(Expr::Literal(LiteralType::Number(
                        previous.literal.parse().unwrap(),
                    )));
                } else {
                    return Box::new(Expr::Literal(LiteralType::String(previous.literal.clone())));
                }
            }
        }

        if self.match_type(vec![TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(
                TokenType::RightParen,
                String::from("Expect ')' after expression."),
            );
            return Box::new(Expr::Grouping(expr));
        }

        Box::new(Expr::Literal(LiteralType::Nil(())))
    }

    fn match_type(&mut self, token_types: Vec<TokenType>) -> bool {
        for t in token_types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        } else {
            let next = self.peek();
            if let Some(token) = next {
                return token.token_type == token_type;
            }
        }

        false
    }

    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current += 1
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        let next = self.peek();

        if let Some(token) = next {
            token.token_type == TokenType::Eof
        } else {
            true
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn previous(&self) -> Option<&Token> {
        self.tokens.get(self.current - 1)
    }

    fn consume(&mut self, token_type: TokenType, message: String) {}
}
