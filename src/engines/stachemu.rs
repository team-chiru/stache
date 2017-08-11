use serde_json::Value;
use std::collections::HashMap;

use error::ExecutionError;
use rule::{ Template, Rule };
use command::{ Engine, Command };

pub type Stachemu = Command<String, Value>;

impl Engine<String, Value> for Stachemu {
    fn decide(template: &Rule, context: &String) -> Self {
        unimplemented!();
    }

    fn execute(self, template: &mut Template, partial: &HashMap<String, Template>, contexts: &Vec<String>) -> Result<Value, ExecutionError> {
        unimplemented!();
    }

    fn render(template: Template, partials: HashMap<String, Template>, contexts: Vec<String>) -> Result<Value, ExecutionError> {
        unimplemented!();
    }
}
