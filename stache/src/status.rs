#[derive(Debug, Clone)]
pub struct Status<T> {
    pub error: Option<T>,
    pub line: i32,
    pub column: i32
}

impl<T> Default for Status<T> {
    fn default() -> Self {
        Status {
            error: None,
            column: 0,
            line: 0,
        }
    }
}

impl<T> Status<T> {
    pub fn throw(&mut self, error: T) {
        self.error = Some(error);
    }

    pub fn updates(&mut self, s: &str) {
        let lines: Vec<&str> = s.lines().collect();
        let lines_len: i32 = lines.len() as i32;

        if lines_len == 1 {
            if s == "\n" {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += lines[0].len() as i32;
            }
        } else if let Some(line) = lines.last() {
            self.column += lines_len;
            self.line = line.len() as i32;
        }
    }
}
