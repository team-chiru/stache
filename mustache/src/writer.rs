#[derive(Debug, Clone)]
pub struct StringWriter {
    pub buffer: String,
    pub is_written: bool
}

impl StringWriter {
    pub fn new() -> Self {
        Self {
            buffer: String::default(),
            is_written: false
        }
    }

    pub fn reset(&mut self) {
        self.is_written = false;
    }

    pub fn write(&mut self, new: &String) {
        self.buffer.push_str(&new);
        self.is_written = true;
    }
}