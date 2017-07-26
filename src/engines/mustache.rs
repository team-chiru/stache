extern crate serde_json;
use self::serde_json::Value;
use std::slice::SliceConcatExt;

use engines::processor::{ Processor, Engine, TemplateEngine };
use error::ExecutionError;
use rule::Rule;

pub struct Builder {
    data: Value,
    output: Vec<String>
}

fn interpolate(key: &String, json: &Value) -> MustacheCommand {
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
            unimplemented!()
        }
    } else {
        Command::Write(String::default())
    }
}

fn interpolate_section(key: &String, json: &Value) -> MustacheCommand {
    let mut data = Some(json);
    let close = Rule::Symbolic("/".to_string(), key.clone());

    if *key != String::default() {
        let path = String::from("/") + &key.replace(".", "/");
        data = data.unwrap().pointer(&path);
    }

    if let Some(json) = data {
        use self::serde_json::Value::*;

        match json.clone() {
            Bool(false) | Null => Command::Skip(close),
            Array(values) => Command::SliceOff(close, values),
            _ => Command::SliceOff(close, vec![json.clone()])
        }
    } else {
        Command::Skip(close)
    }
}

#[derive(Debug, Clone)]
pub enum Command<Input, Output> {
    Skip(Rule),
    SliceOff(Rule, Vec<Input>),
    Import(Rule),
    Write(Output),
    None
}

type MustacheCommand = Command<Value, String>;

impl TemplateEngine<Value, String> for Builder {
    fn configure(json: Value) -> Self {
        Builder {
            data: json,
            output: vec![]
        }
    }

    fn execute(&mut self, p: &mut Processor, rule: &Rule) -> Result<String, ExecutionError> {
        use self::Rule::*;
        let value = self.data.clone();

        // executes the rule symbol
        let command: Command<Value, String> = match *rule {
            Symbolic(ref symbol, ref key) => {
                match symbol.as_ref() {
                    "" => interpolate(key, &value),
                    "#" => interpolate_section(key, &value),
                    "^" => unimplemented!(),
                    "/" => Command::None,
                    ">" => unimplemented!(),
                    "!" => unimplemented!(),
                    _ => unimplemented!()
                }
            },
            Noop(ref symbol) => {
                match symbol.as_ref() {
                    "" => {
                        let value = self.data.clone();
                        interpolate(&String::default(), &value)
                    },
                    "#" => {
                        let close = Rule::Noop("/".to_string());

                        match self.data.clone() {
                            Value::Array(values) => Command::SliceOff(close, values),
                            _ => unimplemented!()
                        }
                    },
                    "/" => Command::None,
                    _ => unimplemented!()
                }
            },
            Default(ref value) => {
                Command::Write(value.clone())
            }
        };

        // executes the post-processing command
        use self::Command::*;

        match command {
            Skip(next_rule) => {
                p.current = p.update_to(&next_rule).unwrap();
            },
            SliceOff(next_rule, slices) => {
                if let Some(section) = p.section_to(&next_rule) {
                    for data in slices {
                        self.output.push(
                            Self::process(section.clone(), data).unwrap()
                        );
                    }
                }
            },
            Import(_) => unimplemented!(),
            Write(value) => {
                self.output.push(value);
                p.current += 1;
            },
            None => {
                p.current += 1;
            }
        }

        Ok(self.output())
    }

    fn output(&self) -> String {
        self.output.join("")
    }
}

impl Engine<Value, String> for Builder {

}
