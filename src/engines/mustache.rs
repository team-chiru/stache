extern crate serde_json;
use self::serde_json::Value;

use error::ExecutionError;
use rule::{ Symbol, Rule, Template };
use command::{ Engine, Command };

fn interpolate(key: &String, json: &Value) -> Mustache {
    let mut data = Some(json);

    if *key != String::default() {
        let path = String::from("/") + &key.replace(".", "/");
        data = data.unwrap().pointer(&path);
    }

    if let Some(value) = data {
        use self::serde_json::Value::*;

        let value = match *value {
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
        };

        if let Some(v) = value {
            Command::Write(v)
        } else {
            Command::None
        }
    } else {
        Command::None
    }
}

fn interpolate_section(key: &String, context: &Value) -> Mustache {
    let path = String::from("/") + &key.replace(".", "/");
    let close = Rule::Symbolic(false, Symbol::from("/"), key.clone());

    if let Some(json) = context.pointer(&path) {
        use self::serde_json::Value::*;

        match json.clone() {
            Bool(false) | Null => Command::Skip(close),
            Array(values) => Command::SliceOff(close, values, true),
            _ => Command::SliceOff(close, vec![json.clone()], true)
        }
    } else {
        Command::Skip(close)
    }
}

fn interpolate_inverted(key: &String, context: &Value) -> Mustache {
    let path = String::from("/") + &key.replace(".", "/");
    let default = vec![context.clone()];
    let close = Rule::Symbolic(false, Symbol::from("/"), key.clone());

    if let Some(json) = context.pointer(&path) {
        use self::serde_json::Value::*;

        match json.clone() {
            Bool(true) => Command::Skip(close),
            Bool(false) | Null => Command::SliceOff(close, default, false),
            Array(values) => {
                if values.is_empty() {
                    Command::SliceOff(close, default, true)
                } else {
                    Command::Skip(close)
                }
            },
            _ => Command::Skip(close)
        }
    } else {
        Command::SliceOff(close, default, false)
    }
}

pub type Mustache = Command<Value, String>;

impl Engine<Value, String> for Mustache {
    fn decide(rule: &Rule, context: &Value) -> Self {
        use self::Rule::*;

        match *rule {
            Symbolic(false, ref symbol, ref key) => {
                match symbol.get() {
                    "" => interpolate(key, context),
                    "#" => interpolate_section(key, context),
                    "^" => interpolate_inverted(key, context),
                    "/" => Command::None,
                    ">" => unimplemented!(),
                    "!" => Command::None,
                    _ => unimplemented!()
                }
            },
            Noop(false, ref symbol) => {
                match symbol.as_ref() {
                    "" => interpolate(&String::default(), context),
                    "#" => {
                        let close = Rule::Noop(false, "/".to_string());

                        match context.clone() {
                            Value::Array(values) => Command::SliceOff(close, values, true),
                            _ => Command::None
                        }
                    },
                    "/" => Command::None,
                    _ => unimplemented!()
                }
            },
            Default(false, ref value) => {
                Command::Write(value.clone())
            },
            _ => Command::None
        }
    }

    fn execute(self, template: &mut Template, contexts: &Vec<Value>) -> Result<String, ExecutionError> {
        use self::Command::*;

        match self {
            Skip(next_rule) => {
                if template.walk(&next_rule).is_none() {
                    Err(ExecutionError::InvalidStatement(
                        String::from("Incomplete template")
                    ))
                } else {
                    Ok(String::default())
                }
            },
            SliceOff(next_rule, slices, is_global_needed) => {
                if let Some(section) = template.split_until(&next_rule) {
                    let mut result = String::default();

                    for slice in slices {
                        let mut contexts = contexts.clone();

                        if is_global_needed {
                            contexts.push(slice);
                        }

                        match Mustache::process_all(section.clone(), contexts) {
                            Ok(value) => result.push_str(&value),
                            Err(error) => return Err(error)
                        }
                    }

                    Ok(result)
                } else {
                    Err(ExecutionError::InvalidStatement(
                        String::from("Incomplete template")
                    ))
                }
            },
            Import(_) => unimplemented!(),
            Write(value) => Ok(value),
            None => {
                Ok(String::default())
            }
        }
    }
}
