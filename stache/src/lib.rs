#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate toml;
extern crate regex;
extern crate heck;

pub mod expr;
pub use self::expr::{ Expression, Command, Delimiter };

mod compiling;
pub use compiling::rule_compiler::{ compiles_raw, RuleCompiler };

pub mod rule;
pub use rule::{ Rule, DefaultRule };

mod template;
pub use self::template::Template;

mod rendering;
pub use self::rendering::RuleEngine;

pub mod status;
pub mod file;
pub mod specs;

pub mod error {
    pub use compiling::error::CompilingError;
    pub use rendering::error::RenderingError;
}
