use super::types::TokenType;

pub struct ScanResult {
    read: i16, // red not reed!
    lines: i16,
    token_to_add: Option<TokenType>
}

impl ScanResult {
    pub fn new() -> Self {
        Self {
            read: 0,
            lines: 0,
            token_to_add: None,
        }
    }

    pub fn inc_read(&mut self) {
        self.read += 1;
    }

    pub fn inc_lines(&mut self) {
        self.lines += 1;
    }

    pub fn inc_lines_by_x(&mut self, x: i16) {
        self.lines += x;
    }

    pub fn inc_read_by_x(&mut self, x: i16) {
        self.read += x;
    }

    pub fn set_token(&mut self, tt: TokenType) {
        self.token_to_add = Some(tt);
    }

    pub fn read(&self) -> i16 {
        self.read
    }

    pub fn lines(&self) -> i16 {
        self.lines
    }

    pub fn token_to_add(&self) -> Option<TokenType> {
        self.token_to_add.clone()
    }
}