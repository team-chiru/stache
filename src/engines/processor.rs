extern crate serde_json;

use rule::{ Rule, Template };
use error::ExecutionError;

pub struct Processor {
    status: Option<ExecutionError>,
    template: Template,
    pub current: i32,
}

impl Processor {
    pub fn new(tmpl: Template) -> Self {
        Processor {
            status: None,
            template: tmpl,

            current: 0
        }
    }

    pub fn get(&mut self, index: i32) -> Option<Rule> {
        self.template.get(index as usize).map(|r| r.clone())
    }

    pub fn find_rule(&mut self, next_rule: &Rule) -> Option<i32> {
        let mut index: usize = 0;
        let mut nested_level = 1;
        let current = self.current as usize;

        let old_rule = match self.template.get(current) {
            Some(rule) => rule,
            None => return None
        };

        let mut current_rule = match self.template.get(current + 1) {
            Some(rule) => rule,
            None => return None
        };

        while current_rule != next_rule && nested_level != 0  {
            if current_rule == next_rule {
                nested_level -= 1;
            } else if current_rule == old_rule {
                nested_level += 1;
            }

            index += 1;
            current_rule = match self.template.get(index) {
                Some(rule) => rule,
                None => return None
            };
        }

        Some(index as i32)
    }

    pub fn section_to(&mut self, next_rule: &Rule) -> Option<Vec<Rule>> {
        if let Some(index) = self.find_rule(&next_rule) {
            let current = self.current as usize;
            self.current = index;
            let index = index as usize;

            let (_, new) = self.template.split_at(current + 1);
            let (section, _) = new.split_at(index - current - 1);

            Some(section.to_vec())
        } else {
            self.status = Some(
                ExecutionError::InvalidStatement(String::from("Incomplete template"))
            );

            None
        }
    }

    pub fn update_to(&mut self, next_rule: &Rule) -> Option<i32> {
        if let Some(index) = self.find_rule(&next_rule) {
            self.current = index;
            Some(index)
        } else {
            self.status = Some(
                ExecutionError::InvalidStatement(String::from("Incomplete template"))
            );

            None
        }
    }
}

impl Iterator for Processor {
    type Item = Rule;

    fn next(&mut self) -> Option<Rule> {
        let current_index = self.current;
        self.get(current_index)
    }
}

pub trait Engine<Input, Output> {
    fn new(data: Input) -> Self;
    fn process(&self, tmpl: Template, data: Input) -> Result<Output, ExecutionError>;
}
