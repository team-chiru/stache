use { Template, Partials };
use super::error::CompilingError;
use super::parser::Parser;
use super::trailer::Trailer;
use std::collections::HashMap;

use toml;
use serde;

use std::fmt::Debug;
pub trait TemplateCompiler where for<'de> Self: serde::Deserialize<'de> + Clone + Default + PartialEq {
    fn compiles_template(input: String) -> Result<Template<Self>, CompilingError>;
    fn compiles_partial(partials_input: HashMap<String, String>) -> Result<Partials<Self>, CompilingError>;
    fn compiles_all(input: String, partials_input: HashMap<String, String>) -> Result<(Template<Self>, Partials<Self>), CompilingError>;

    fn compiles(rules_file: &str, optional_input: Option<String>, optional_partials: Option<HashMap<String, String>>) -> Result<(Template<Self>, Partials<Self>), CompilingError> {
        let descr = match toml::from_str(&rules_file) {
            Ok(descr) => descr,
            Err(_) => return Err(
                CompilingError::InvalidStatement(
                    String::from("deserialize config")
                )
            )
        };

        let mut parser = Parser::init(&descr)?;

        let main = match optional_input {
            Some(input) => {
                let default_main = Trailer::trails(parser.parses(input)?)?;
                default_main.from_default::<Self>(&descr)
            },
            _ => Template::default()
        };


        let mut partials = Partials::default();

        if let Some(partials_input) = optional_partials {
            for (key, input) in partials_input {
                let default_partial = Trailer::trails(parser.parses(input)?)?;

                partials.insert(key, default_partial.from_default::<Self>(&descr));
            }
        }

        Ok((main, partials))
    }
}
