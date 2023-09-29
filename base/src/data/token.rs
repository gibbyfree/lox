use super::types::TokenType;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct Token {
    pub tt: TokenType,
    pub lexeme: String,
    pub line: i16,
}

impl Token {
    pub fn new(tt: TokenType, lexeme: String, line: i16) -> Self {
        Self { tt, lexeme, line }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} {}", self.tt, self.lexeme, self.line)
    }
}
