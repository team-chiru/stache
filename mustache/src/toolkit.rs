#![allow(dead_code)]
use serde_json::Value;

#[derive(Debug, Clone)]
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
pub fn interpolate_section(key: &String, context: &Value, global: &Vec<Value>) -> Vec<Vec<Value>> {
    let path = String::from("/") + &key.replace(".", "/");
    let mut slices: Vec<Value> = vec![];

    if "." == key {
        if let Value::Array(mut values) = context.clone() {
            slices.append(&mut values);
        }
    } else if let Some(json) = context.pointer(&path) {
        use self::Value::*;

        match json.clone() {
            Bool(false) | Null => {},
            Array(mut values) => {
                if !values.is_empty() {
                    slices.append(&mut values);
                }
            },
            _ => slices.push(json.clone())
        }
    }

    let mut new_contexts = vec![];
    for slice in slices {
        let mut new_context = global.clone();

        new_context.push(slice); // global context is needed
        new_contexts.push(new_context);
    }

    new_contexts
}

pub fn interpolate_inverted(key: &String, context: &Value, global: &Vec<Value>) -> Vec<Vec<Value>> {
    let mut new_contexts = vec![];
    let path = String::from("/") + &key.replace(".", "/");

    if let Some(json) = context.pointer(&path) {
        use self::Value::*;

        match json.clone() {
            Bool(false) | Null => {
                new_contexts.push(vec![context.clone()]);
            }
            Array(values) => {
                if values.is_empty() {
                    let mut new_context = global.clone();
                    new_context.push(context.clone()); // global context is needed
                    new_contexts.push(new_context);
                }
            },
            _ => {}
        }
    } else {
        let mut new_context = global.clone();
        new_context.push(context.clone()); // global context is needed
        new_contexts.push(new_context);
    }

    new_contexts
}
