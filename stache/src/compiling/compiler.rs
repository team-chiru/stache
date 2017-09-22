use { Template, Partials };
use super::error::CompilingError;
use super::parser::Parser;
use super::trailer::Trailer;
use std::collections::HashMap;

use toml;
use serde;

pub trait TemplateCompiler where for<'de> Self: serde::Deserialize<'de> + Clone + Default + PartialEq {
    fn compiles(input: String, partials_input: HashMap<String, String>) -> Result<(Template<Self>, Partials<Self>), CompilingError>;

    fn compiles_with_raw(config: &str, input: String, partials_input: HashMap<String, String>) -> Result<(Template<Self>, Partials<Self>), CompilingError> {
        let descr = match toml::from_str(&config) {
            Ok(descr) => descr,
            Err(_) => return Err(
                CompilingError::InvalidStatement(
                    String::from("deserialize config")
                )
            )
        };

        let mut parser = Parser::init(&descr)?;
        let default = parser.parses(input)?;

        let trailed = Trailer::trails(default)?;

        Ok((trailed.from_default::<Self>(&descr), Partials::default()))
    }
}
