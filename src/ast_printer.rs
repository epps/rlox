use crate::{
    expr::{Expr, LiteralType},
    visitor::Visitor,
};

pub struct AstPrinter {}

impl AstPrinter {
    pub fn print(&self, expr: Box<Expr>) -> String {
        self.visit(expr)
    }
    fn parenthesize(&self, name: String, exprs: Vec<Box<Expr>>) -> String {
        let mut str = String::from("(");

        str.push_str(&name);

        for e in exprs {
            str.push(' ');
            str.push_str(&self.visit(e));
        }

        str.push(')');

        str
    }
}

impl Visitor<String> for AstPrinter {
    fn visit(&self, expr: Box<Expr>) -> String {
        match *expr {
            Expr::Binary {
                left,
                operator,
                right,
            } => self.parenthesize(operator.literal, vec![left, right]),
            Expr::Grouping(e) => self.parenthesize(String::from("group"), vec![e]),
            Expr::Literal(lit) => match lit {
                LiteralType::Number(n) => n.to_string(),
                LiteralType::String(s) => s,
                LiteralType::True(t) => t.to_string(),
                LiteralType::False(f) => f.to_string(),
                LiteralType::Nil(()) => String::from("nil"),
            },
            Expr::Unary { operator, right } => self.parenthesize(operator.literal, vec![right]),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{expr::Expr, token::Token};
    use crate::{expr::LiteralType, token::TokenType};

    use super::AstPrinter;

    #[test]
    fn it_pretty_prints() {
        let expr = Box::new(Expr::Binary {
            left: Box::new(Expr::Unary {
                operator: Token::new(TokenType::Minus, String::from("-"), String::from("-"), 1),
                right: Box::new(Expr::Literal(LiteralType::Number(123.0))),
            }),
            operator: Token::new(TokenType::Star, String::from("*"), String::from("*"), 1),
            right: Box::new(Expr::Grouping(Box::from(Expr::Literal(
                LiteralType::Number(45.67),
            )))),
        });

        let ast_printer = AstPrinter {};
        let output = ast_printer.print(expr);

        assert_eq!(output, String::from("(* (- 123) (group 45.67))"));
    }
}
