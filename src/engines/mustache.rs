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
}

impl RuleEngine<String> for Builder {
    fn execute(&mut self, rule: &Rule) -> Result<NextRule, ExecutionError> {
        match *rule {
            Rule::Symbolic(ref symbol, ref key) => {
                println!("{:?}", key);

                match symbol.as_ref() {
                    "" => Ok(None),
                    "#" => Ok(None),
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
