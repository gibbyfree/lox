use crate::data::types::TokenType;

#[derive(Clone)]
pub struct Token {
    tt: TokenType,
    lexeme: String,
    line: i16,
}

impl Token {
    pub fn new(tt: TokenType, lexeme: String, line: i16) -> Self {
        Self { tt, lexeme, line }
    }
}
    