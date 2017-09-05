#![feature(slice_concat_ext)]

extern crate regex;

#[macro_use] extern crate lazy_static;

#[macro_use] extern crate serde_derive;
extern crate serde_json;

pub mod expr;

pub mod compiling;
pub use compiling::compile;

pub mod execution;

pub mod implementations;

pub mod rule;
pub mod status;
pub mod file;
pub mod specs;
