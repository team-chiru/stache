use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum RenderingError {
    InvalidStatement(String)
}

impl fmt::Display for RenderingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RenderingError::InvalidStatement(ref state) =>
                write!(f, "InvalidStatement error: {}", state)
        }
    }
}

impl error::Error for RenderingError {
    fn description(&self) -> &str {
        match *self {
            RenderingError::InvalidStatement { .. } => "an invalid sequence of states has occurred",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            _ => None,
        }
    }
}
