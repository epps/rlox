use crate::expr::Expr;

pub trait Visitor<T> {
    fn visit(&self, expr: Box<Expr>) -> T;
}
