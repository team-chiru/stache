use rule::{ Rule, Template };
use error::ExecutionError;

pub type NextRule = Option<Rule>;

pub trait TemplateEngine<U> {
    fn execute(&mut self, &Rule) -> Result<NextRule, ExecutionError>;
    fn output(&self) -> U;
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

    fn update_to_next(&mut self, next: Option<Rule>) {
        self.current += 1;
    }
}

impl Iterator for Processor {
    type Item = Rule;

    fn next(&mut self) -> Option<Rule> {
        self.template.get(self.current as usize).map(|r| r.clone())
    }
}

pub fn process<T, U>(tmpl: Template, engine: &mut T) -> Result<U, ExecutionError>
where T: TemplateEngine<U> {
    let mut p = Processor::new(tmpl);

    while let Some(rule) = p.next() {
        match engine.execute(&rule) {
            Err(err) => return Err(err),
            Ok(next) => p.update_to_next(next)
        }
    }

    Ok(engine.output())
}
