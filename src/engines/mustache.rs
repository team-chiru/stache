extern crate serde_json;
use self::serde_json::Value;

use processor::{ RuleEngine, NextRule };
use error::ExecutionError;
use rule::Rule;

pub struct Builder {
    data: Value,
    output: String
}

impl Builder {
    pub fn configure(json: Value) -> Self {
        Builder {
            data: json,
            output: "".to_string()
        }
    }

    fn interpolate(&mut self, key: &String) -> Result<NextRule, ExecutionError> {
        let path = String::from("/") + &key.replace(".", "/");

        if let Some(json_value) = self.data.pointer(&path) {
            use self::serde_json::Value::*;
            let err = ExecutionError::InvalidStatement(
                "Invalid JSON structure".to_string()
            );

            let value = match json_value.clone() {
                Bool(b) => {
                    Some(
                        if b {
                            "true".to_string()
                        } else {
                            "false".to_string()
                        }
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
                Ok(None)
            } else {
                Err(err)
            }
        } else {
            Err(
                ExecutionError::InvalidStatement(
                    "No matching key".to_string()
                )
            )
        }
    }

    fn interpolate_section(&mut self, key: &String) -> Result<NextRule, ExecutionError> {
        let close_rule = Rule::Symbolic("/".to_string(), key.clone());

        if let Some(json_value) = self.data.get(key) {
            use self::serde_json::Value::*;

            match json_value.clone() {
                Bool(b) => unimplemented!(),
                Number(n) => unimplemented!(),
                String(s) => unimplemented!(),
                Bool(b) => unimplemented!(),
                Array(arr) => unimplemented!(),
                Object(obj) => unimplemented!(),
                Null => unimplemented!()
            };
        }

        Ok(Some(close_rule))
    }
}

impl RuleEngine<String> for Builder {
    fn execute(&mut self, rule: &Rule) -> Result<NextRule, ExecutionError> {
        match *rule {
            Rule::Symbolic(ref symbol, ref key) => {
                match symbol.as_ref() {
                    "" => self.interpolate(key),
                    "#" => self.interpolate_section(key),
                    "^" => Ok(None),
                    "/" => Ok(None),
                    ">" => Ok(None),
                    "!" => Ok(None),
                    _ => Err(
                        ExecutionError::InvalidStatement(
                            "Incorrect symbol".to_string()
                        )
                    )
                }
            },
            Rule::Default(ref value) => {
                self.output.push_str(value);
                Ok(None)
            }
        }
    }

    fn output(&self) -> String {
        self.output.clone()
    }
}
