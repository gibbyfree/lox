use crate::data::token::Token;

trait Expr { }

trait Visitor<R> { 
    fn visit_binary_expr(&self, binary: &Binary) -> R;
    fn visit_grouping_expr(&self, grouping: &Grouping) -> R;
    fn visit_literal_expr(&self, literal: &Literal) -> R;
    fn visit_unary_expr(&self, unary: &Unary) -> R;
}

trait Visitable<R> {
    fn accept(&self, visitor: &dyn Visitor<R>) -> R;
}

struct Binary {
    left: Box<dyn Expr>,
    operator: Token,
    right: Box<dyn Expr>
}

impl<R> Visitable<R> for Binary {
    fn accept(&self, visitor: &dyn Visitor<R>) -> R {
        visitor.visit_binary_expr(self)
    }
}

struct Grouping {
    expr: Box<dyn Expr>
}

impl<R> Visitable<R> for Grouping {
    fn accept(&self, visitor: &dyn Visitor<R>) -> R {
        visitor.visit_grouping_expr(self)
    }
}

struct Unary {
    operator: Token,
    right: Box<dyn Expr>
}

impl<R> Visitable<R> for Unary {
    fn accept(&self, visitor: &dyn Visitor<R>) -> R {
        visitor.visit_unary_expr(self)
    }
}

struct Literal {
    value: Token
}

impl<R> Visitable<R> for Literal {
    fn accept(&self, visitor: &dyn Visitor<R>) -> R {
        visitor.visit_literal_expr(self)
    }
}