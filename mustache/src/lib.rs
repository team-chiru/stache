#[macro_use] extern crate serde_derive;

extern crate serde_json;
use serde_json::Value;

extern crate stache;
use stache::{ Template, RuleEngine, RuleCompiler, compiles_raw };
use stache::error::{ RenderingError, CompilingError };

use std::collections::HashMap;

mod toolkit;
use self::toolkit::*;

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub enum Mustache {
    Interpolation(String),
    EscapedInterpolation(String),
    Section(String),
    InvertedSection(String),
    Close(String),
    Partial(String),
    Comment(String),
    Default(String)
}

impl RuleCompiler<Mustache> for Mustache {
    fn compiles(raw: &String) -> Result<Template<Mustache>, CompilingError> {
        let descr = include_str!("../mustache.toml");

        compiles_raw(&descr, &raw)
    }
}

impl RuleEngine<Mustache, Value, String> for Mustache {
    fn render(template: Template<Mustache>, partials: HashMap<String, Template<Mustache>>, contexts: Vec<Value>) -> Result<String, RenderingError> {
        let mut output = String::default();
        let mut template = template.clone();

        while let Some(ref rule) = template.next() {
            let mut context_stack = contexts.iter().rev();

            while let Some(context) = context_stack.next() {
                use self::Mustache::*;

                let mut is_written = false;

                match *rule {
                    Interpolation(ref value) => {
                        let key = match value.as_ref() {
                            "." => String::default(),
                            _ => value.clone()
                        };

                        if let Some(write) = interpolate(&key, context) {
                            output.push_str(&write);
                            is_written = true;
                        }
                    },
                    EscapedInterpolation(ref value) => {
                        unimplemented!()
                    }
                    Section(ref value) => {
                        let cmd = match value.as_ref() {
                            "." => {
                                if let Value::Array(values) = context.clone() {
                                    //Extractor::new(&String::default(), values, true))
                                    Some(values)
                                } else {
                                    None
                                }
                            },
                            _ => unimplemented!()
                        };
                    },
                    InvertedSection(ref value) => {}
                    Close(ref value) => {}
                    Partial(ref value) => {}
                    Comment(ref value) => {}
                    Default(ref value) => {}
                }

                if is_written { // FIXME || rule.is_dotted()
                    break;
                }
            }
        }

        unimplemented!()
    }
}
