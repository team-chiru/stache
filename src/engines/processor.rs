use rule::{ Rule, Template };
use error::ExecutionError;

pub type NextRule = Option<Rule>;

pub trait TemplateEngine<Input, Output> {
    fn configure(Input) -> Self;
    fn execute(&mut self, &Rule) -> Result<NextRule, ExecutionError>;
    fn output(&self) -> Output;
}

struct Processor {
    template: Template,
    current: i32,
}

impl Processor {
    fn new(tmpl: Template) -> Self {
        Processor {
            template: tmpl,
            current: 0
        }
    }

    fn get(&mut self, index: i32) -> Option<Rule> {
        self.template.get(index as usize).map(|r| r.clone())
    }

    fn update_to_next(&mut self, next: Option<Rule>) {
        if next.is_some() {
            let mut index = self.current + 1;
            let current_index = self.current;
            let current = self.get(current_index);
            let mut nested_level = 0;

            while nested_level != 0 && self.get(index) != next {
                if self.get(index) == current {
                    nested_level += 1;
                } else if self.get(index) == next && nested_level > 0 {
                    nested_level -= 1;
                }

                index += 1;
            }

            self.current = index;
        } else {
            self.current += 1;
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

pub trait Engine<Input, Output> where Self: TemplateEngine<Input, Output> {
    fn process(&mut self, tmpl: Template) -> Result<Output, ExecutionError> {
        let mut p = Processor::new(tmpl);

        while let Some(rule) = p.next() {
            match self.execute(&rule) {
                Err(err) => return Err(err),
                Ok(next) => p.update_to_next(next)
            }
        }

        Ok(self.output())
    }
}
