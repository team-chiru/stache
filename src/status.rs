#[derive(Debug, Clone)]
pub struct Status<T> {
    pub error: Option<T>,
    pub line_index: i32,
    pub column_index: i32
}
