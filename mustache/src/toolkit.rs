#![allow(dead_code)]
use serde_json::Value;

pub struct Writter<Output> {
    pub buffer: Output,
    pub is_written: bool
}

impl<Output> Writter<Output> where Output: Default {
    pub fn new() -> Self {
        Writter {
            buffer: Output::default(),
            is_written: false
        }
    }

    pub fn reset(&mut self) {
        self.is_written = false;
    }
}

impl Writter<String> {
    pub fn write(&mut self, new: &String) {
        self.buffer.push_str(&new);
        self.is_written = true;
    }
}

pub fn interpolate(key: &String, json: &Value) -> Option<String> {
    let mut data = Some(json);

    if *key != String::default() {
        let path = String::from("/") + &key.replace(".", "/");
        data = data.unwrap().pointer(&path);
    }

    if let Some(value) = data {
        use self::Value::*;

        match *value {
            Bool(b) => {
                Some(
                    if b { "true".to_string() } else { "false".to_string() }
                )
            },
            String(ref s) => Some(s.clone()),
            Number(ref n) => {
                if let Some(s) = n.as_i64() {
                    Some(s.to_string())
                } else if let Some(s) = n.as_u64() {
                    Some(s.to_string())
                } else if let Some(s) = n.as_f64() {
                    Some(s.to_string())
                } else {
                    None
                }
            },
            _ => None
        }
    } else {
        None
    }
}

// needs global each time
pub fn interpolate_section(key: &String, context: &Value) -> Option<Vec<Value>> {
    let path = String::from("/") + &key.replace(".", "/");

    if let Some(json) = context.pointer(&path) {
        use self::Value::*;

        match json.clone() {
            Bool(false) | Null => None,
            Array(values) => {
                if values.is_empty() {
                    None
                } else {
                    Some(values)
                }
            },
            _ => Some(vec![json.clone()])
        }
    } else {
        None
    }
}

fn interpolate_inverted(key: &String, context: &Value) -> Option<Vec<Value>> {
    let path = String::from("/") + &key.replace(".", "/");
    let default = vec![context.clone()];

    if let Some(json) = context.pointer(&path) {
        use self::Value::*;

        match json.clone() {
            Bool(true) => None,
            Bool(false) | Null => Some(default),
            Array(values) => {
                if values.is_empty() {
                    Some(default)
                } else {
                    None
                }
            },
            _ => None
        }
    } else {
        Some(default)
    }
}

use stache::{ Template, Partials, TemplateEngine };
use stache::error::{ RenderingError };
use Mustache;

#[derive(Debug, Clone)]
pub struct Extractor {
    skip: Mustache,
    slices: Vec<Value>
}

impl Extractor {
    pub fn new(key: String) -> Self {
        Extractor {
            skip: Mustache::Close(key),
            slices: vec![]
        }
    }

    pub fn append(&mut self, contexts: &mut Vec<Value>) {
        self.slices.append(contexts);
    }

    pub fn extract(&self, template: &mut Template<Mustache>, partials: Partials<Mustache>, global: &Vec<Value>) -> Result<String, RenderingError> {
        let mut writter = Writter::new();

        if let Some(section) = template.split_until(&self.skip) {
            for slice in &self.slices {
                println!("{:?}", slice);
                let mut new_contexts = global.clone();
                new_contexts.push(slice.clone()); // global context is needed

                println!("{:?}", section);
                println!("{:?}", new_contexts);
                match Mustache::render(section.clone(), partials.clone(), new_contexts) {
                    Ok(write) => {
                        writter.write(&write);
                    },
                    Err(error) => return Err(error)
                }

                writter.reset();
            }
        } else {
            return Err(RenderingError::InvalidStatement(
                String::from("Incomplete template")
            ));
        }

        Ok(writter.buffer)
    }
}
