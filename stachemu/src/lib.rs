#[macro_use] extern crate serde_derive;

extern crate serde_json;
use serde_json::Value;

extern crate stache;
use stache::{ Template, TemplateEngine, TemplateCompiler, Partials };
use stache::testing::{ Pool };
use stache::rule::{ Rule, Map };
use stache::error::{ RenderingError, CompilingError };
use stache::expr::Description;

use std::collections::HashMap;

mod toolkit;
use self::toolkit::*;

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub enum Stachemu {
    Interpolation(String),
    Default(String)
}

pub type Test = Pool<Stachemu, Value, String>;

impl Default for Stachemu {
    fn default() -> Self { Stachemu::Default(String::default()) }
}

impl TemplateCompiler for Mustache {
    fn get_description() -> Description {
        Description::from_toml(&include_str!("../Stachemu.toml"))
    }
}

impl TemplateEngine<Stachemu, Value, String> for Stachemu {
    fn render(template: Template<Stachemu>, partials: Partials<String, Template>, contexts: Vec<String>) -> Result<Value, RenderingError> {
        let mut output = Value::Null;
        let mut tmpl = template.clone();
        
        let mut context = match contexts.get(0) {
            Some(ctx) => ctx.clone(),
            None => return Err(
                RenderingError::InvalidStatement(
                    "Only one context can be rendered!".to_string()
                )
            )
        };

        while let Some(rule) = tmpl.next() {
            use self::Rule::*;
            use self::Command::*;

            let engine: Stachemu = match rule {
                Interpolation(ref key) => {
                    let mut new_map = Map::new();

                    if let Some(value) = interpolate(&context) {
                        new_map.insert(key.to_string(), Value::String(value));
                    }

                    Write(Value::Object(new_map))
                }
                Default(ref value) => {
                    let (to_match, new_context) = context.split_at(value.len());

                    if is_matching(value, to_match) {
                        Command::Write(Value::Object(Map::new()))
                    } else {
                        Command::Write(Value::Null)
                    }
                },
                _ => unimplemented!()
            };

            match engine {
                Skip(next_rule) => unimplemented!(),
                Extract(next_rule, slices, is_global_needed) => unimplemented!(),
                Import(key) => unimplemented!(),
                Write(mut value) => {
                    let out = reshape_interpolation(value, &template);

                    if let Value::Object(map) = out {
                        for (key, value) in map {
                            merge_into(&mut output, &key, &value);

                            if let Value::String(eaten) = value.clone() {
                                context.drain(..eaten.len());
                            }
                        }

                        if let Rule::Default(eaten) = rule.clone() {
                            context.drain(..eaten.len());
                        }

                    } else if out == Value::Null {
                        return Ok(out);
                    }
                },
                None => unimplemented!()
            }
        }

        Ok(output)
    }
}