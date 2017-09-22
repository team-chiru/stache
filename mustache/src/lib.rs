#![allow(unused_variables)]
#[macro_use] extern crate serde_derive;

extern crate serde_json;
use serde_json::Value;

extern crate stache;
use stache::{ Template, TemplateEngine, TemplateCompiler, Partials };
use stache::testing::{ Pool };
use stache::rule::Rule;
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

pub type Test = Pool<Mustache, Value, String>;

impl Default for Mustache {
    fn default() -> Self {
        Mustache::Default(String::default())
    }
}

impl Rule for Mustache {
    fn is_dotted(&self) -> bool {
        use self::Mustache::*;

        match *self {
            Interpolation(ref key) if key.contains(".") => true,
            EscapedInterpolation(ref key) if key.contains(".") => true,
            Section(ref key) if key.contains(".") => true,
            InvertedSection(ref key) if key.contains(".") => true,
            _ => false
        }
    }
}

impl TemplateCompiler for Mustache {
    fn compiles(raw: String, partials_raw: HashMap<String, String>) -> Result<(Template<Mustache>, Partials<Mustache>), CompilingError> {
        Self::compiles_with_raw(&include_str!("../mustache.toml"), raw, HashMap::new())
    }
}

impl TemplateEngine<Mustache, Value, String> for Mustache {
    fn render(template: Template<Mustache>, partials: Partials<Mustache>, contexts: Vec<Value>) -> Result<String, RenderingError> {
        let mut writter = Writter::new();
        let mut template = template.clone();
        let global = contexts.clone();

        while let Some(ref rule) = template.next() {
            let mut context_stack = global.iter().rev();

            while let Some(context) = context_stack.next() {
                use self::Mustache::*;

                match *rule {
                    Interpolation(ref key) => {
                        let key = match key.as_ref() {
                            "." => String::default(),
                            _ => key.clone()
                        };

                        if let Some(write) = interpolate(&key, context) {
                            writter.write(&write);
                        }
                    },
                    EscapedInterpolation(_) => {
                        unimplemented!()
                    }
                    Section(ref key) => {
                        /*
                        let mut slices: Vec<Value> = vec![];

                        match key.clone().as_ref() {
                            "." => {
                                if let Value::Array(mut values) = context.clone() {
                                    slices.append(&mut values);
                                }
                            },
                            _ => {
                                if let Some(mut values) = interpolate_section(&key, &context) {
                                    slices.append(&mut values);
                                }
                            }
                        }

                        if let Some(section) = template.split_until(&Mustache::Close(key.clone())) {
                            for slice in slices {
                                let mut new_contexts = global.clone();
                                new_contexts.push(slice); // global context is needed

                                println!("{:?}", section);
                                println!("{:?}", new_contexts);
                                match Mustache::render(section.clone(), partials.clone(), new_contexts.clone()) {
                                    Ok(write) => {
                                        writter.write(&write);
                                    },
                                    Err(error) => return Err(error)
                                }
                            }
                        } else {
                            return Err(RenderingError::InvalidStatement(
                                String::from("Incomplete template")
                            ));
                        }
                        */
                        let mut extr = Extractor::new(key.clone());
                        //println!("{:?}", context);

                        match key.clone().as_ref() {
                            "." => {
                                if let Value::Array(mut values) = context.clone() {
                                    extr.append(&mut values);
                                }
                            },
                            _ => {
                                if let Some(mut values) = interpolate_section(&key, &context) {
                                    extr.append(&mut values);
                                }
                            }
                        }

                        match extr.extract(&mut template, partials.clone(), &global) {
                            Ok(write) => writter.write(&write),
                            Err(err) => return Err(err)
                        }
                    },


                    InvertedSection(_) => {

                    },
                    Close(_) => {}
                    Partial(_) => {}
                    Comment(_) => {}
                    Default(ref value) => {
                        writter.write(&value);
                    }
                }

                if writter.is_written || rule.is_dotted() {
                    writter.reset();
                    break;
                }
            }
        }

        Ok(writter.buffer)
    }
}
