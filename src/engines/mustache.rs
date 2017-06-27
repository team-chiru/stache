extern crate serde_json;
use self::serde_json::Value;

use processor::{ RuleEngine, NextRule };
use error::ExecutionError;
use rule::Rule;

pub struct Builder {}

impl RuleEngine<Value> for Builder {
    fn configure() -> Self {
        Builder {}
    }

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
                println!("{:?}", value);
                Ok(None)
            }
        }
    }

    fn output(&self) -> Value {
        Value::Null
    }
}
