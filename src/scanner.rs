use crate::data::token::Token;
use crate::data::types::TokenType;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize, // first char in scanned lexeme
    current: usize, // char considered
    line: i16 // source line of current
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self { 
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1
        }
    }

    fn scan_tokens(mut self) -> Vec<Token> {
        while !is_at_end {
            self.start = self.current;
            scan_token();
        }

        self.tokens.push(Token::new(TokenType::End, "", self.line));
        self.tokens
    }

    fn is_at_end() -> bool {
        self.current >= self.source.len()
    }
}