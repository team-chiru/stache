extern crate serde_json;
use self::serde_json::Value;

use std::collections::HashMap;

use execution::{ ExecutionError, ;
use rule::{ Rule, DefaultRule, Template };
use command::{ Engine };

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

        unimplemented!()
        /*
        if let Some(v) = value {
            Command::Write(v)
        } else {
            Command::None
        }
    } else {
        Command::None
    }
    */
}

fn interpolate_section(key: &String, context: &Value) -> Mustache {
    let path = String::from("/") + &key.replace(".", "/");

    unimplemented!()

    /*
    let close = Rule::Symbolic(Symbol::from("/"), key.clone());
    if let Some(json) = context.pointer(&path) {
        use self::serde_json::Value::*;

        match json.clone() {
            Bool(false) | Null => Command::Skip(close),
            Array(values) => {
                if values.is_empty() {
                    Command::Skip(close)
                } else {
                    Command::Extract(close, values, true)
                }
            },
            _ => Command::Extract(close, vec![json.clone()], true)
        }
    } else {
        Command::Skip(close)
    }
    */
}

fn interpolate_inverted(key: &String, context: &Value) -> Mustache {
    let path = String::from("/") + &key.replace(".", "/");
    let default = vec![context.clone()];

/*
let close = Rule::Symbolic(Symbol::from("/"), key.clone());
    if let Some(json) = context.pointer(&path) {
        use self::serde_json::Value::*;

        match json.clone() {
            Bool(true) => Command::Skip(close),
            Bool(false) | Null => Command::Extract(close, default, false),
            Array(values) => {
                if values.is_empty() {
                    Command::Extract(close, default, true)
                } else {
                    Command::Skip(close)
                }
            },
            _ => Command::Skip(close)
        }
    } else {
        Command::Extract(close, default, false)
    }
    */
}

//pub type Mustache = Command<Value, String>;
struct MustacheEngine<'symbol> {
    template: Template<'symbol>,
    partials: HashMap<String, Template<'symbol>>,
    context: Vec<Value>,
    output: String
}

impl<'symbol> Engine<Value, String> for MustacheEngine<'symbol> {
    fn render(template: Template, partials: HashMap<String, Template>, contexts: Vec<Value>) -> Result<String, ExecutionError> {
        let mut output = String::default();
        let mut template = template.clone();

        while let Some(rule) = template.next() {
            let mut context_stack = contexts.iter().rev();

/*
            while let Some(context) = context_stack.next() {
                use self::Rule::*;
                use self::Command::*;

                let mut is_written = false;

                let cmd = match rule {
                    Symbolic(ref symbol, ref key) => {
                        match symbol.get() {
                            "" => interpolate(key, context),
                            "#" => interpolate_section(key, context),
                            "^" => interpolate_inverted(key, context),
                            "/" => Command::None,
                            ">" => Command::Import(key.clone()),
                            "!" => Command::None,
                            _ => unimplemented!()
                        }
                    },
                    Iterator(ref symbol) => {
                        match symbol.as_ref() {
                            "" => interpolate(&String::default(), context),
                            "#" => {
                                let close = Rule::Iterator("/".to_string());

                                match context.clone() {
                                    Value::Array(values) => Command::Extract(close, values, true),
                                    _ => Command::None
                                }
                            },
                            "/" => Command::None,
                            _ => unimplemented!()
                        }
                    },
                    Default(ref value) => {
                        Command::Write(value.clone())
                    },
                    _ => Command::None
                };

                match cmd {
                    Skip(next_rule) => {
                        if template.walk_until(&next_rule).is_none() {
                            return Err(ExecutionError::InvalidStatement(String::from("Incomplete template")));
                        }
                    },
                    Extract(next_rule, slices, is_global_needed) => {
                        if let Some(section) = template.split_until(&next_rule) {
                            for slice in slices {
                                let mut contexts = contexts.clone();

                                if is_global_needed {
                                    contexts.push(slice);
                                }

                                match Mustache::render(section.clone(), partials.clone(), contexts) {
                                    Ok(value) => {
                                        output.push_str(&value);
                                        is_written = true;
                                    },
                                    Err(error) => return Err(error)
                                }
                            }
                        } else {
                            return Err(ExecutionError::InvalidStatement(String::from("Incomplete template")));
                        }
                    },
                    Import(key) => {
                        if let Some(template) = partials.get(&key) {
                            let mut new_contexts = contexts.clone();

                            if let Some(context) = contexts.last() {
                                new_contexts = vec![context.clone()];
                            }

                            match Mustache::render(template.clone(), partials.clone(), new_contexts) {
                                Ok(value) => {
                                    output.push_str(&value);
                                    is_written = true;
                                },
                                Err(error) => return Err(error)
                            }
                        }
                    },
                    Write(value) => {
                        output.push_str(&value);
                        is_written = true;
                    },
                    None => {}
                }

                if is_written || rule.is_dotted() {
                    break;
                }
            }
            */
        }

        Ok(output)
    }
}
