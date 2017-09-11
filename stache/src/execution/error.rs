use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ExecutionError {
    InvalidStatement(String)
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExecutionError::InvalidStatement(ref state) =>
                write!(f, "InvalidStatement error: {}", state)
        }
    }
}

impl error::Error for ExecutionError {
    fn description(&self) -> &str {
        match *self {
            ExecutionError::InvalidStatement { .. } => "an invalid sequence of states has occurred",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            _ => None,
        }
    }
}
