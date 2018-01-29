use stache::Writer;

#[derive(Debug, Clone)]
pub struct StringWriter {
    pub buffer: String,
    pub is_written: bool
}

impl Writer<String> for StringWriter {
    fn new() -> Self {
        Self {
            buffer: String::default(),
            is_written: false
        }
    }

    fn reset(&mut self) {
        self.is_written = false;
    }

    fn write(&mut self, new: &String) {
        self.buffer.push_str(&new);
        self.is_written = true;
    }
}