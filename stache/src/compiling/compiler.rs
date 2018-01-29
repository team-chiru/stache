use { Template, Partials, Descriptor };
use super::error::CompilingError;
use super::parser::Parser;
use super::trailer::Trailer;
use std::collections::HashMap;

use serde;

pub trait TemplateCompiler where for<'de> Self: serde::Deserialize<'de> + Clone + Default + PartialEq {
    fn get_descriptor() -> Descriptor;

    fn compiles_template(input: String) -> Result<Template<Self>, CompilingError> {
        match Self::compiles(Self::get_descriptor(), Some(input), None) {
            Ok((tmpl, _)) => Ok(tmpl),
            Err(err) => Err(err)
        }
    }

    fn compiles_partial(partials_input: HashMap<String, String>) -> Result<Partials<Self>, CompilingError> {
        match Self::compiles(Self::get_descriptor(), None, Some(partials_input)) {
            Ok((_, partials)) => Ok(partials),
            Err(err) => Err(err)
        }
    }

    fn compiles_all(input: String, partials_input: HashMap<String, String>) -> Result<(Template<Self>, Partials<Self>), CompilingError> {
        Self::compiles(Self::get_descriptor(), Some(input), Some(partials_input))
    }

    fn compiles(descriptor: Descriptor, optional_input: Option<String>, optional_partials: Option<HashMap<String, String>>) -> Result<(Template<Self>, Partials<Self>), CompilingError> {
        let description = descriptor.get();

        if description.is_none() {
            return Err(
                CompilingError::InvalidStatement(
                    String::from("invalid descriptor")
                )
            )
        }
        
        let descr = description.unwrap();
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
