extern crate regex;

#[macro_use] extern crate serde_derive;
extern crate serde_json;

pub mod expr;
pub use self::expr::{ Expression, Symbol, Delimiter };

pub mod compiling;
pub use self::compiling::{ Matcher, RuleMatching };

pub mod execution;

pub mod rule;
pub use rule::{ Rule, DefaultRule };

pub mod status;
pub mod file;
pub mod specs;

pub use compiling::compiles_template;

pub fn compile() {

}
