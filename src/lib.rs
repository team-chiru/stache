extern crate regex;

pub mod compiler;
pub mod rule;
pub mod error;

pub mod status;
pub type CompilingStatus = status::Status<error::CompilingError>;
