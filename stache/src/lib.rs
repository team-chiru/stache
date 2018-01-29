#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate toml;
extern crate regex;
extern crate heck;

pub mod expr;
pub use self::expr::{ Expression, Command, Delimiter };

mod descriptor;
pub use self::descriptor::Descriptor;

mod compiling;
pub use compiling::compiler::TemplateCompiler;

pub mod rule;
pub use rule::{ DefaultRule };

mod template;
pub use self::template::Template;
pub use self::template::Partials;

mod rendering;
pub use self::rendering::TemplateEngine;

pub mod status;
pub mod file;
pub mod testing;

pub mod error {
    pub use compiling::error::CompilingError;
    pub use rendering::error::RenderingError;
}
