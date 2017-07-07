extern crate serde_json;
use self::serde_json::Value;

use engines::processor::{ Processor, Engine, TemplateEngine };
use error::ExecutionError;
use rule::Rule;

pub struct Builder {
    data: Value,
    output: String
}

impl Builder {
    fn interpolate(&mut self, json: &Value) -> MustacheCommand<Value> {
        use self::serde_json::Value::*;

        let value = match *json {
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
            self.output.push_str(&v);
            MustacheCommand::None
        } else {
            unimplemented!()
        }
    }

    fn interpolate_with_key(&mut self, key: &String) -> MustacheCommand<Value> {
        let path = String::from("/") + &key.replace(".", "/");
        let data = self.data.clone();
        let json = data.pointer(&path);

        if let Some(value) = json {
            self.interpolate(value)
        } else {
            MustacheCommand::None
        }
    }

    fn interpolate_section(&self, json: &Value, close: Rule) -> MustacheCommand<Value> {
        use self::MustacheCommand::*;
        use self::serde_json::Value::*;

        println!("{:?}", json);

        match json.clone() {
            Bool(false) | Null => Skip(close),
            Array(values) => SliceOff(close, values),
            _ => SliceOff(close, vec![json.clone()])
        }
    }

    fn interpolate_section_with_key(&mut self, symbol: &str, key: &String) -> MustacheCommand<Value> {
        let close = Rule::Symbolic(symbol.to_string(), key.clone());
        let path = String::from("/") + &key.replace(".", "/");

        if let Some(json) = self.data.pointer(&path) {
            self.interpolate_section(json, close)
        } else {
            MustacheCommand::Skip(close)
        }
    }
}

#[derive(Debug, Clone)]
pub enum MustacheCommand<Input> {
    Skip(Rule),
    SliceOff(Rule, Vec<Input>),
    Import(Rule),
    None
}

impl TemplateEngine<Value, String> for Builder {
    fn configure(json: Value) -> Self {
        Builder {
            data: json,
            output: String::default()
        }
    }

    fn execute(&mut self, p: &mut Processor, rule: &Rule) -> Result<String, ExecutionError> {
        use self::Rule::*;
        use self::serde_json::Value::*;

        // executes the rule symbol
        let command = match *rule {
            Symbolic(ref symbol, ref key) => {
                match symbol.as_ref() {
                    "" => self.interpolate_with_key(key),
                    "#" => self.interpolate_section_with_key("/", key),
                    "^" => MustacheCommand::None,
                    "/" => MustacheCommand::None,
                    ">" => MustacheCommand::None,
                    "!" => MustacheCommand::None,
                    _ => unimplemented!()
                }
            },
            Noop(ref symbol) => {
                match symbol.as_ref() {
                    "" => {
                        let value = self.data.clone();
                        self.interpolate(&value)
                    },
                    "#" => {
                        let close = Rule::Noop("/".to_string());

                        match self.data.clone() {
                            Array(values) => SliceOff(close, values),
                            _ => unimplemented!()
                        }
                    },
                    "/" => MustacheCommand::None,
                    _ => unimplemented!()
                }
            },
            Default(ref value) => {
                self.output.push_str(value);
                MustacheCommand::None
            }
        };

        // executes the post-processing command
        use self::MustacheCommand::*;

        match command {
            Skip(next_rule) => {
                p.current = p.update_to(&next_rule).unwrap();
            },
            SliceOff(next_rule, slices) => {
                if let Some(section) = p.section_to(&next_rule) {
                    for data in slices {
                        self.output.push_str(
                            &Self::process(section.clone(), data).unwrap()
                        );
                    }
                }
            },
            Import(_) => unimplemented!(),
            None => {
                p.current += 1;
            }
        }

        Ok(self.output.clone())
    }

    fn output(&self) -> String {
        self.output.clone()
    }
}

impl Engine<Value, String> for Builder {}
