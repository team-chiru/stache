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
