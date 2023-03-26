use crate::data::payload::ScanResult;
use crate::data::token::Token;
use crate::data::types::TokenType;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,   // first char in scanned lexeme
    current: usize, // char considered
    line: i16,      // source line of current
}

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut m = HashMap::new();
        m.insert("and", TokenType::And);
        m.insert("class", TokenType::Class);
        m.insert("else", TokenType::Else);
        m.insert("false", TokenType::False);
        m.insert("for", TokenType::For);
        m.insert("fun", TokenType::Fun);
        m.insert("if", TokenType::If);
        m.insert("nil", TokenType::Nil);
        m.insert("or", TokenType::Or);
        m.insert("print", TokenType::Print);
        m.insert("return", TokenType::Return);
        m.insert("super", TokenType::Super);
        m.insert("this", TokenType::This);
        m.insert("true", TokenType::True);
        m.insert("var", TokenType::Var);
        m.insert("while", TokenType::While);
        m
    };
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
            let res = Self::scan_token(
                &self.source,
                self.current,
                self.start,
                self.line,
                &mut tokens,
            );
            self.line += res.lines();
            self.current += res.read() as usize;
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
    ) -> ScanResult {
        let mut res = ScanResult::new();
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
                res.inc_read();
            }
            '=' => {
                let t = if Self::cond_advance(source, current, '=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                Self::add_token(t, tokens, start, current, source, line);
                res.inc_read();
            }
            '>' => {
                let t = if Self::cond_advance(source, current, '=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Equal
                };
                Self::add_token(t, tokens, start, current, source, line);
                res.inc_read();
            }
            '<' => {
                let t = if Self::cond_advance(source, current, '=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Equal
                };
                Self::add_token(t, tokens, start, current, source, line);
                res.inc_read();
            }
            '/' => {
                if Self::cond_advance(source, current, '/') {
                    while Self::peek(current, source) != '\n'
                        && !Self::is_at_end(current, source.len())
                    {
                        res.inc_read();
                    }
                } else {
                    Self::add_token(TokenType::Slash, tokens, start, current, source, line);
                    res.inc_read();
                };
            }
            '"' => {
                let sub_res = Self::string(current, source, start);
                res.inc_lines_by_x(sub_res.lines());
                res.inc_read_by_x(sub_res.read());
                if let Some(tt) = sub_res.token_to_add() {
                    Self::add_token(tt, tokens, start, current, source, line);
                }
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                let sub_res = Self::number(current, source, start);
                res.inc_read_by_x(sub_res.read());
                if let Some(tt) = sub_res.token_to_add() {
                    Self::add_token(tt, tokens, start, current, source, line);
                }
            }
            ' ' | '\t' | '\r' => (),
            '\n' => res.inc_lines(),
            _ => {
                if Self::is_alpha(Self::advance(source, current)) {
                    let sub_res = Self::identifier_or_keyword(current, source, start);
                    res.inc_read_by_x(sub_res.read());
                    if let Some(tt) = sub_res.token_to_add() {
                        Self::add_token(tt, tokens, start, current, source, line);
                    }
                } else {
                    println!("surface lexical error to main later");
                }
            }
        }
        res.inc_read();
        res
    }

    // SCANRESULT HELPERS //

    fn identifier_or_keyword(current: usize, source: &str, start: usize) -> ScanResult {
        let mut res = ScanResult::new();
        let mut loc_current = current;

        while Self::is_alpha_numeric(Self::peek(loc_current, source)) {
            res.inc_read();
            loc_current += 1;
        }

        let val = &source[start..loc_current];
        if let Some(kw) = KEYWORDS.get(val) {
            // tis a keyword
            res.set_token(kw.clone());
        } else {
            // identifier
            res.set_token(TokenType::Identifier); // weird we don't save the string here?
        }

        res
    }

    fn number(current: usize, source: &str, start: usize) -> ScanResult {
        let mut res = ScanResult::new();
        let mut loc_current = current;

        while Self::peek(loc_current, source).is_ascii_digit() {
            res.inc_read();
            loc_current += 1;
        }

        // fractional part
        if Self::peek(loc_current, source) == '.' && Self::double_peek(loc_current, source).is_ascii_digit() {
            res.inc_read(); // decimal
            loc_current += 1;

            while Self::peek(loc_current, source).is_ascii_digit() {
                res.inc_read();
                loc_current += 1;
            }
        }

        // create token
        let val = &source[start..loc_current];
        res.set_token(TokenType::Number(val.parse::<f32>().expect("failed to parse number")));

        res
    }

    fn string(current: usize, source: &str, start: usize) -> ScanResult {
        let mut res = ScanResult::new(); // let's just append to top-level response later
        let mut loc_current = current; // local current
        while Self::peek(loc_current, source) != '"' && !Self::is_at_end(loc_current, source.len())
        {
            if Self::peek(loc_current, source) == '\n' {
                res.inc_lines();
            }
            res.inc_read();
            loc_current += 1;
        }

        if Self::is_at_end(loc_current, source.len()) {
            println!("Unterminated string."); // surface error here
            return res;
        }

        // one more 'advance' for the closing quote
        res.inc_read();
        loc_current += 1;

        // trim surrounding quotes
        let val = &source[start + 1..loc_current - 1];
        res.set_token(TokenType::String(val.to_string()));

        res
    }

    // PEEKING HELPERS //

    // peek ahead by 1 character
    fn peek(current: usize, source: &str) -> char {
        if Self::is_at_end(current, source.len()) {
            return '\0';
        }
        return source.chars().nth(current).expect("peek machine broke");
    }

    // peek ahead by 2 characters
    fn double_peek(current: usize, source: &str) -> char {
        if Self::is_at_end(current + 1, source.len()) {
            return '\0';
        }
        return source.chars().nth(current + 1).expect("double peek machine broke");
    }

    fn is_at_end(current: usize, source_len: usize) -> bool {
        current >= source_len
    }

    fn cond_advance(source: &str, current: usize, expected: char) -> bool {
        if Self::is_at_end(current, source.len()) {
            return false;
        }
        let next = source.chars().nth(current + 1).expect("cond advance");
        if next != expected {
            return false;
        }
        true
    }

    fn advance(source: &str, current: usize) -> char {
        source.chars().nth(current + 1).expect("current is borked")
    }

    // MISC. HELPERS //
    // snake case identifiers are valid
    fn is_alpha(c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn is_alpha_numeric(c: char) -> bool {
        Self::is_alpha(c) || c.is_ascii_digit()
    }

    // ADD TOKEN //
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
