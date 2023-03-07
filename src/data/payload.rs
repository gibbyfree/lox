pub struct ScanResult {
    read: i16, // red not reed!
    lines: i16,
}

impl ScanResult {
    pub fn new() -> Self {
        Self {
            read: 0,
            lines: 0,
        }
    }

    pub fn inc_read(&mut self) {
        self.read += 1;
    }

    pub fn inc_lines(&mut self) {
        self.lines += 1;
    }

    pub fn read(&self) -> i16 {
        self.read
    }

    pub fn lines(&self) -> i16 {
        self.lines
    }
}