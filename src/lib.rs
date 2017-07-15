#![feature(slice_concat_ext)]

extern crate regex;
#[macro_use] extern crate serde_derive;

mod compiler;
pub use compiler::compile;

pub mod engines;

pub mod rule;
pub mod error;
pub mod status;
pub mod file;
pub mod specs;
