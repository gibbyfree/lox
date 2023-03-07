use crate::data::token::Token;
use crate::data::types::TokenType;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,   // first char in scanned lexeme
    current: usize, // char considered
    line: i16,      // source line of current
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn scan_tokens(mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        std::mem::swap(&mut tokens, &mut self.tokens);

        while Self::is_at_end(self.current, self.source.len()) {
            self.start = self.current;
            Self::scan_token(
                &self.source,
                self.current,
                self.start,
                self.line,
                &mut tokens,
            );
        }

        tokens.push(Token::new(TokenType::End, String::from(""), self.line));
        self.tokens = tokens;
        self.tokens.clone()
    }

    fn scan_token(
        source: &str,
        current: usize,
        start: usize,
        line: i16,
        tokens: &mut Vec<Token>,
    ) -> i16 {
        let mut read: i16 = 0; // red not reed!
        match Self::advance(source, current) {
            '(' => Self::add_token(TokenType::LeftParen, tokens, start, current, source, line),
            ')' => Self::add_token(TokenType::RightParen, tokens, start, current, source, line),
            '{' => Self::add_token(TokenType::LeftBrace, tokens, start, current, source, line),
            '}' => Self::add_token(TokenType::RightBrace, tokens, start, current, source, line),
            ',' => Self::add_token(TokenType::Comma, tokens, start, current, source, line),
            '.' => Self::add_token(TokenType::Dot, tokens, start, current, source, line),
            '-' => Self::add_token(TokenType::Minus, tokens, start, current, source, line),
            '+' => Self::add_token(TokenType::Plus, tokens, start, current, source, line),
            ';' => Self::add_token(TokenType::Semicolon, tokens, start, current, source, line),
            '*' => Self::add_token(TokenType::Star, tokens, start, current, source, line),
            '!' => {
                let t = if Self::cond_advance(source, current, '=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                Self::add_token(t, tokens, start, current, source, line);
                read += 1
            },
            '=' => {
                let t = if Self::cond_advance(source, current, '=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                Self::add_token(t, tokens, start, current, source, line);
                read += 1
            },
            '>' => {
                let t = if Self::cond_advance(source, current, '=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Equal
                };
                Self::add_token(t, tokens, start, current, source, line);
                read += 1
            },
            '<' => {
                let t = if Self::cond_advance(source, current, '=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Equal
                };
                Self::add_token(t, tokens, start, current, source, line);
                read += 1
            }
            _ => println!("surface lexical error to main later"),
        }
        read + 1
    }

    // helpers

    fn is_at_end(current: usize, source_len: usize) -> bool {
        current >= source_len
    }

    fn cond_advance(source: &str, current: usize, expected: char) -> bool {
        if Self::is_at_end(current, source.len()) { return false; }
        let next = source.chars().nth(current + 1).expect("cond advance");
        if next != expected { return false; }
        true
    }

    fn advance(source: &str, current: usize) -> char {
        source.chars().nth(current + 1).expect("current is borked")
    }

    // no need for multiple token fns when tokentype can contain literals
    fn add_token(
        t: TokenType,
        tokens: &mut Vec<Token>,
        start: usize,
        current: usize,
        source: &str,
        line: i16,
    ) {
        let text = source
            .get(start..current)
            .expect("current or start is borked");
        tokens.push(Token::new(t, String::from(text), line));
    }
}
