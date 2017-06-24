use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum CompilingError {
    InvalidStatement(String)
}

use self::CompilingError::*;
impl fmt::Display for CompilingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InvalidStatement(ref state) =>
                write!(f, "InvalidStatement error: {}", state)
        }
    }
}

impl error::Error for CompilingError {
    fn description(&self) -> &str {
        match *self {
            InvalidStatement { .. } => "an invalid sequence of states has occurred",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            _ => None,
        }
    }
}
