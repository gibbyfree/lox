use base::data::expr::{Binary, Expr, Grouping, Literal, Unary, Visitor};

struct AstPrinter;
impl AstPrinter {
    fn print(&self, expr: Expr) -> String {
        expr.accept(self)
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(&self, binary: &Binary) -> String {
        parenthesize(&binary.operator.lexeme, &[&binary.left, &binary.right])
    }

    fn visit_grouping_expr(&self, grouping: &Grouping) -> String {
        parenthesize("group", &[&grouping.expr])
    }

    fn visit_literal_expr(&self, literal: &Literal) -> String {
        literal.value.lexeme.to_string()
    }

    fn visit_unary_expr(&self, unary: &Unary) -> String {
        parenthesize(&unary.operator.lexeme, &[&unary.right])
    }
}

fn parenthesize(name: &str, exprs: &[&Expr]) -> String {
    let mut s = String::from("(");
    s.push_str(name);
    for expr in exprs {
        s.push(' ');
        s.push_str(&expr.accept(&AstPrinter));
    }
    s.push(')');
    s
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use base::data::{token::Token, types::TokenType};

    #[test]
    fn test_book_example() -> Result<()> {
        let expression = Expr::Binary(Binary {
            left: Box::new(Expr::Unary(Unary {
                operator: Token::new(TokenType::Minus, String::from("-"), 1),
                right: Box::new(Expr::Literal(Literal {
                    value: Token::new(TokenType::Number(123.0), String::from("123"), 1),
                })),
            })),
            operator: Token::new(TokenType::Star, String::from("*"), 1),
            right: Box::new(Expr::Grouping(Grouping {
                expr: Box::new(Expr::Literal(Literal {
                    value: Token::new(TokenType::Number(45.67), String::from("45.67"), 1),
                })),
            })),
        });

        let ast_printer = AstPrinter;
        let actual = ast_printer.print(expression);
        assert_eq!(actual, "(* (- 123) (group 45.67))");

        Ok(())
    }
}
