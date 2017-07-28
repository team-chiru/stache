extern crate serde_json;
use self::serde_json::Value;

use engines::processor::{ Processor, Engine };
use error::ExecutionError;
use rule::{ Symbol, Rule, Template };

pub struct Builder {
    global: Value
}

impl Builder {
    fn execute(&self, p: &mut Processor, command: MustacheCommand) -> String {
        use self::Command::*;
        let mut result = String::default();

        match command {
            Skip(next_rule) => {
                p.current = p.update_to(&next_rule).unwrap();
            },
            SliceOff(next_rule, slices) => {
                if let Some(section) = p.section_to(&next_rule) {
                    for data in slices {
                        result.push_str(
                            &self.process(section.clone(), vec![data]).unwrap()
                        );
                    }
                }
            },
            Import(_) => unimplemented!(),
            Write(value) => {
                result.push_str(&value);
                p.current += 1;
            },
            None => {
                p.current += 1;
            }
        }

        result
    }
}

fn merge(target: Value, sample: &Value) -> Value {
    use self::serde_json::Value::*;

    match (target.clone(), sample.clone()) {
        (Object(map), Object(sample)) => {
            let mut map = map.clone();

            for (key, value) in sample {
                map.insert(key, value);
            }

            Value::Object(map)
        },
        _ => unimplemented!()
    }
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

fn interpolate_section(key: &String, local: &Value, global: &Value) -> MustacheCommand {
    let mut data = None;
    let close = Rule::Symbolic(Symbol::from("/"), key.clone());
    let path = String::from("/") + &key.replace(".", "/");

    if *key != String::default() {
        if let Some(resolved) = local.pointer(&path) {
            data = Some(resolved.clone());
        }

        /*
         * if local context doesn't match, retry on merged global context
         * @see Deeply Nested Contexts
         */
        if !data.is_some() {
            if let Some(resolved) = global.pointer(&path) {
                data = Some(merge(resolved.clone(), local));
            }
        }
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

fn trail(value: &String) -> String {
    value.clone()
}

fn decide(rule: &Rule, data: &Value, global: &Value) -> MustacheCommand {
    use self::Rule::*;

    match *rule {
        Symbolic(ref symbol, ref key) => {
            match symbol.get() {
                "" => interpolate(key, &data),
                "#" => interpolate_section(key, &data, &global),
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
                    interpolate(&String::default(), &data)
                },
                "#" => {
                    let close = Rule::Noop("/".to_string());

                    match data.clone() {
                        Value::Array(values) => Command::SliceOff(close, values),
                        _ => unimplemented!()
                    }
                },
                "/" => Command::None,
                _ => unimplemented!()
            }
        },
        Default(ref value) => {
            Command::Write(trail(value))
        }
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

impl Engine<Vec<Value>, String> for Builder {
    fn new(data: Vec<Value>) -> Self {
        if let Some(context) = data.get(0) {
            Builder { global: context.clone() }
        } else {
            Builder { global: Value::Null }
        }
    }

    fn process(&self, tmpl: Template, values: Vec<Value>) -> Result<String, ExecutionError> {
        let mut output = String::default();
        let global = self.global.clone();

        for data in values {
            let mut processor = Processor::new(tmpl.clone());
            let mut partial = String::default();

            while let Some(rule) = processor.next() {
                let cmd: Command<Value, String> = decide(&rule, &data, &global);
                partial.push_str(&self.execute(&mut processor, cmd));
            }

            if partial != String::default() {
                output.push_str(&partial.clone());
                break;
            }
        }

        Ok(output)
    }
}
