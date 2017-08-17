use serde_json::{ Value, Map };
use std::collections::HashMap;

use error::ExecutionError;
use rule::{ Template, Rule };
use command::{ Engine, Command };

pub type Stachemu = Command<String, Value>;

fn is_matching(template: &str, to_match: &str) -> bool {
    let mut value_to_match = template.chars();
    let mut chars_to_match = to_match.chars();

    while let Some(value_c) = value_to_match.next() {
        if let Some(match_c) = chars_to_match.next() {
            if value_c != match_c {
                return false;
            }
        }
    }

    true
}

fn interpolate(context: &String) -> Option<String> {
    let mut value = String::default();
    let mut context = context.chars().rev().collect::<String>();

    while let Some(c) = context.pop() {
        if c.is_whitespace() {
            break;
        } else {
            value.push(c);
        }
    }

    if value != String::default() {
        Some(value)
    } else {
        None
    }
}

fn reshape_interpolation(to_reshape: Value, template: &Template) -> Value {
    let mut new_map = Map::new();
    let next_index = template.now() + 1;

    if let Value::Object(map) = to_reshape.clone() {
        if map.len() != 1 {
            return to_reshape;
        }

        if let Some(Rule::Default(false, next)) = template.get(next_index) {
            for (key, value) in map {
                if let Value::String(mut content) = value.clone() {
                    for next_char in next.chars() {
                        if let Some(last_match) = content.pop() {
                            if last_match != next_char {
                                content.push(last_match);
                                break;
                            }
                        }
                    }

                    new_map.insert(key.clone(), Value::String(content));
                }
            }
        }
    }

    if new_map.is_empty() {
        to_reshape
    } else {
        Value::Object(new_map)
    }
}

fn merge_into(into: &Value, key: &String, value: &Value) -> Value {
    use std::slice::SliceConcatExt;

    match *into {
        Value::Object(ref map) => {
            if key.contains(".") {
                let mut keys: Vec<&str> = key.split(".").collect();
                if let Some(first) = keys.clone().get(0) {
                    let mut is_merged = false;

                    for (old_key, mut old_value) in map {
                        if old_key == first {
                            keys.remove(0);
                            merge_into(old_value, &keys.join("."), value);
                            is_merged = true;
                            break;
                        }
                    }
                }
            } else {
                //map.insert(key.clone(), value.clone());
            }

            unimplemented!()
        },
        Value::Null => {
            if key.contains(".") {
                let mut keys: Vec<&str> = key.split(".").collect();
                let mut new_json = Value::Object(Map::new());

                for key in keys {
                    new_json = merge_into(&new_json, &String::from(key), value);
                }
            } else {

            }

            unimplemented!()
        },
        _ => unimplemented!()
    }
}

impl Engine<String, Value> for Stachemu {
    fn create(rule: &Rule, context: &String) -> Self {
        use self::Rule::*;

        match *rule {
            Symbolic(false, ref symbol, ref key) => {
                match symbol.get() {
                    "" => {
                        let mut new_map = Map::new();

                        if let Some(value) = interpolate(context) {
                            new_map.insert(key.to_string(), Value::String(value));
                        }

                        Command::Write(Value::Object(new_map))
                    },
                    "#" => unimplemented!(),
                    "^" => unimplemented!(),
                    "/" => unimplemented!(),
                    ">" => unimplemented!(),
                    "!" => unimplemented!(),
                    _ => unimplemented!()
                }
            },
            Noop(false, ref symbol) => {
                match symbol.as_ref() {
                    "" => unimplemented!(),
                    "#" => unimplemented!(),
                    "/" => unimplemented!(),
                    _ => unimplemented!()
                }
            },
            Default(false, ref value) => {
                let (to_match, new_context) = context.split_at(value.len());

                if is_matching(value, to_match) {
                    Command::Write(Value::Object(Map::new()))
                } else {
                    Command::Write(Value::Null)
                }
            },
            _ => unimplemented!()
        }
    }

    fn execute(self, template: &mut Template, partial: &HashMap<String, Template>, contexts: &Vec<String>) -> Result<Value, ExecutionError> {
        use self::Command::*;

        match self {
            Skip(next_rule) => unimplemented!(),
            Extract(next_rule, slices, is_global_needed) => unimplemented!(),
            Import(key) => unimplemented!(),
            Write(mut value) => Ok(reshape_interpolation(value, template)),
            None => unimplemented!()
        }
    }

    fn render(template: Template, partials: HashMap<String, Template>, contexts: Vec<String>) -> Result<Value, ExecutionError> {
        let mut output = Value::Null;
        let mut tmpl = template.clone();

        let mut context = match contexts.get(0) {
            Some(ctx) => ctx.clone(),
            None => return Err(
                ExecutionError::InvalidStatement(
                    "Only one context can be rendered!".to_string()
                )
            )
        };

        while let Some(rule) = tmpl.next() {
            let engine = Stachemu::create(&rule, &context);

            match engine.execute(&mut tmpl, &partials, &contexts) {
                Ok(value) => {
                    if let Value::Object(map) = value {
                        for (key, value) in map {
                            merge_into(&mut output, &key, &value);

                            if let Value::String(eaten) = value.clone() {
                                context.drain(..eaten.len());
                            }
                        }

                        if let Rule::Default(false, eaten) = rule.clone() {
                            context.drain(..eaten.len());
                        }

                    } else if value == Value::Null {
                        return Ok(value)
                    }
                },
                Err(error) => return Err(error)
            }
        }

        Ok(output)
    }
}
