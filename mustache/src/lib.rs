#![allow(unused_variables)]
#[macro_use] extern crate serde_derive;

extern crate serde_json;
use serde_json::Value;

extern crate stache;
use stache::{ Template, TemplateEngine, TemplateCompiler };
use stache::{ Descriptor, Partials };
use stache::testing::Pool;
use stache::rule::Rule;
use stache::error::{ RenderingError };
use stache::expr::{ Directive, Delimiter, RuleHeap, Description };
use stache::file;

mod toolkit;
use self::toolkit::*;

mod writer;
use self::writer::StringWriter;

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
    fn get_descriptor() -> Descriptor {
        Descriptor::from_toml(&file::read("Mustache.toml").unwrap())
    }
}

impl TemplateEngine<Mustache, Value, String> for Mustache {
    fn render(template: Template<Mustache>, partials: Partials<Mustache>, contexts: Vec<Value>) -> Result<String, RenderingError> {
        let mut writer = StringWriter::new();
        let mut template = template.clone();
        let global = contexts.clone();

        while let Some(ref rule) = template.next() {
            let mut context_stack = global.iter().rev();

            while let Some(context) = context_stack.next() {
                use self::Mustache::*;

                match *rule {
                    Interpolation(ref key) => {
                        if let Some(write) = interpolate(&key, context) {
                            let mut encoded = String::default();

                            write.chars().for_each(|c| match c {
                                '>' => encoded.push_str("&gt;"),
                                '<' => encoded.push_str("&lt;"),
                                '&' => encoded.push_str("&amp;"),
                                '\'' => encoded.push_str("&#39;"),
                                '"' => encoded.push_str("&quot;"),
                                _ => encoded.push(c)
                            });

                            writer.write(&encoded);
                        }
                    },
                    EscapedInterpolation(ref key) => {
                        if let Some(write) = interpolate(&key, context) {
                            writer.write(&write);
                        }
                    }
                    Section(ref key) => {
                        let close = Mustache::Close(key.clone());

                        if let Some(section) = template.split_until(&close) {
                            for new_context in interpolate_section(&key, &context, &global) {
                                match Mustache::render(section.clone(), partials.clone(), new_context) {
                                    Ok(write) => writer.write(&write),
                                    Err(error) => return Err(error)
                                }
                            }
                        } else {
                            return Err(RenderingError::InvalidStatement(
                                String::from("Incomplete template")
                            ));
                        }
                    },
                    InvertedSection(ref key) => {
                        let close = Mustache::Close(key.clone());

                        if let Some(section) = template.split_until(&close) {
                            for new_context in interpolate_inverted(&key, &context, &global) {
                                match Mustache::render(section.clone(), partials.clone(), new_context) {
                                    Ok(write) => writer.write(&write),
                                    Err(error) => return Err(error)
                                }
                            }
                        } else {
                            return Err(RenderingError::InvalidStatement(
                                String::from("Incomplete template")
                            ));
                        }
                    },
                    Close(_) | Comment(_) => {},
                    Partial(ref key) => {
                        if let Some(template) = partials.get(key) {
                            let mut new_contexts = contexts.clone();

                            if let Some(context) = contexts.last() {
                                new_contexts = vec![context.clone()];
                            }

                            match Mustache::render(template.clone(), partials.clone(), new_contexts) {
                                Ok(write) => writer.write(&write),
                                Err(error) => return Err(error)
                            }
                        }
                    },
                    Default(ref value) => {
                        writer.write(&value);
                    }
                }

                if writer.is_written || rule.is_dotted() {
                    writer.reset();
                    break;
                }
            }
        }

        Ok(writer.buffer)
    }
}
