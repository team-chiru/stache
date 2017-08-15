use serde_json::{ Value, Map };
use std::collections::HashMap;

use error::ExecutionError;
use rule::{ Template, Rule };
use command::{ Engine, Command };

#[derive(Debug, Clone)]
pub struct Stachemu {
    own: Command<String, Value>,
    buffer: i32
}

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

impl Engine<String, Value> for Stachemu {
    fn create(rule: &Rule, context: &String) -> Self {
        use self::Rule::*;

        match *rule {
            Symbolic(false, ref symbol, ref key) => {
                match symbol.get() {
                    "" => {
                        let mut new_context = context.clone();
                        let mut buffer = context.len();
                        let mut map = Map::new();

                        if let Some(value) = interpolate(context) {
                            new_context.drain(..buffer).collect::<String>();
                            buffer = value.len();
                            map.insert(key.to_string(), Value::String(value));
                        }

                        Stachemu {
                            own: Command::Write(Value::Object(map)),
                            buffer: buffer as i32
                        }
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
                    Stachemu {
                        own: Command::Write(Value::Object(Map::new())),
                        buffer: value.len() as i32
                    }
                } else {
                    Stachemu {
                        own: Command::Write(Value::Null),
                        buffer: 0
                    }
                }
            },
            _ => unimplemented!()
        }
    }

    fn execute(self, template: &mut Template, partial: &HashMap<String, Template>, contexts: &Vec<String>) -> Result<Value, ExecutionError> {
        use self::Command::*;

        match self.own {
            Skip(next_rule) => unimplemented!(),
            Extract(next_rule, slices, is_global_needed) => unimplemented!(),
            Import(key) => unimplemented!(),
            Write(value) => Ok(value),
            None => unimplemented!()
        }
    }

    fn render(template: Template, partials: HashMap<String, Template>, contexts: Vec<String>) -> Result<Value, ExecutionError> {
        let mut output = Map::new();
        let mut tmpl = template.clone();

        while let Some(rule) = tmpl.next() {
            let mut context_stack = contexts.iter().rev();

            while let Some(context) = context_stack.next() {
                let cmd = Stachemu::create(&rule, &context);
                let mut is_written = false;

                match cmd.execute(&mut tmpl, &partials, &contexts) {
                    Ok(value) => {
                        if let Value::Object(map) = value {
                            for (key, value) in map {
                                output.insert(key, value);
                            }

                            is_written = true;
                        }
                    },
                    Err(error) => return Err(error)
                }

                if is_written {
                    break;
                }
            }
        }

        Ok(Value::Object(output))
    }
}
