#![feature(slice_concat_ext)]
#![allow(unused_variables)]

#[macro_use] extern crate serde_derive;

extern crate serde_json;
use serde_json::{ Value, Map };

extern crate stache;
use stache::{ Descriptor, Template, TemplateEngine, TemplateCompiler, Partials };
use stache::testing::{ Pool };
use stache::error::{ RenderingError };
use stache::expr::{ Directive, Delimiter, RuleHeap, Description };

mod toolkit;
use self::toolkit::*;

mod writer;
use self::writer::ObjectWriter;

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub enum Stachemu {
    Interpolation(String),
    Default(String)
}

pub type Test = Pool<Stachemu, String, Value>;

impl Default for Stachemu {
    fn default() -> Self { Stachemu::Default(String::default()) }
}

impl TemplateCompiler for Stachemu {
    fn get_descriptor() -> Descriptor {
        Descriptor::from_toml(&file::read("Stachemu.toml"))
    }
}

impl TemplateEngine<Stachemu, String, Value> for Stachemu {
    fn render(template: Template<Stachemu>, partials: Partials<Stachemu>, contexts: Vec<String>) -> Result<Value, RenderingError> {
        let mut output = Value::Null;
        let mut tmpl = template.clone();
        let mut writer = ObjectWriter::new(template);
        let mut result = Value::Null;
        
        let mut context = match contexts.get(0) {
            Some(ctx) => ctx.clone(),
            None => return Err(
                RenderingError::InvalidStatement(
                    "Only one context can be rendered!".to_string()
                )
            )
        };

        while let Some(ref rule) = tmpl.next() {
            use self::Stachemu::*;
            let current_rule = rule.clone();

            match *rule {
                Interpolation(ref key) => {
                    let mut new_map = Map::new();

                    if let Some(value) = interpolate(&context) {
                        new_map.insert(key.to_string(), Value::String(value));
                    }

                    result = Value::Object(new_map);
                },
                Default(ref value) => {
                    let (to_match, new_context) = context.split_at(value.len());

                    if is_matching(value, to_match) {
                        result = Value::Object(Map::new());
                    } else {
                        result = Value::Null;
                    }
                },
                _ => unimplemented!()
            };

            writer.write(result, &mut context, current_rule);
        }

        Ok(writer.buffer)
    }
}