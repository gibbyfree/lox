use crate::data::token::Token;

pub trait Expr { }

struct Binary {
    left: Box<dyn Expr>,
    operator: Token,
    right: Box<dyn Expr>
}

struct Grouping {
    expr: Box<dyn Expr>
}

struct Unary {
    operator: Token,
    right: Box<dyn Expr>
}

struct Literal {
    value: Token
}