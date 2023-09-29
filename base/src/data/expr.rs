use crate::data::token::Token;

pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}

impl Expr {
    pub fn accept<R>(&self, visitor: &dyn Visitor<R>) -> R {
        match self {
            Expr::Binary(binary) => visitor.visit_binary_expr(binary),
            Expr::Grouping(grouping) => visitor.visit_grouping_expr(grouping),
            Expr::Literal(literal) => visitor.visit_literal_expr(literal),
            Expr::Unary(unary) => visitor.visit_unary_expr(unary),
        }
    }
}

pub trait Visitor<R> {
    fn visit_binary_expr(&self, binary: &Binary) -> R;
    fn visit_grouping_expr(&self, grouping: &Grouping) -> R;
    fn visit_literal_expr(&self, literal: &Literal) -> R;
    fn visit_unary_expr(&self, unary: &Unary) -> R;
}

pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct Grouping {
    pub expr: Box<Expr>,
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct Literal {
    pub value: Token,
}
