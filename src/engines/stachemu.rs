use serde_json::{ Value, Map };
use std::collections::HashMap;

use error::ExecutionError;
use rule::{ Template, Rule };
use command::{ Engine, Command };

pub type Stachemu = Command<String, Value>;

impl Engine<String, Value> for Stachemu {
    fn decide(rule: &Rule, context: &String) -> Self {
        use self::Rule::*;

        match *rule {
            Symbolic(false, ref symbol, ref key) => {
                match symbol.get() {
                    "" => unimplemented!(),
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
            Default(false, ref value) => unimplemented!(),
            _ => unimplemented!()
        }
    }

    fn execute(self, template: &mut Template, partial: &HashMap<String, Template>, contexts: &Vec<String>) -> Result<Value, ExecutionError> {
        use self::Command::*;

        match self {
            Skip(next_rule) => unimplemented!(),
            SliceOff(next_rule, slices, is_global_needed) => unimplemented!(),
            Import(key) => unimplemented!(),
            Write(value) => unimplemented!(),
            None => unimplemented!()
        }
    }

    fn render(template: Template, partials: HashMap<String, Template>, contexts: Vec<String>) -> Result<Value, ExecutionError> {
        let mut output = Map::new();
        let mut tmpl = template.clone();

        while let Some(rule) = tmpl.next() {
            let mut context_stack = contexts.iter().rev();

            while let Some(context) = context_stack.next() {
                let cmd = Stachemu::decide(&rule, &context);
                let mut is_written = false;

                match cmd.execute(&mut tmpl, &partials, &contexts) {
                    Ok(value) => {
                        if let Value::Object(map) = value  {
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
