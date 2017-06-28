use rule::Rule;
use error::ExecutionError;

pub type NextRule = Option<Rule>;

pub trait RuleEngine<U> {
    fn execute(&mut self, &Rule) -> Result<NextRule, ExecutionError>;
    fn output(&self) -> U;
}

pub fn process<T, U>(rules: Vec<Rule>, engine: &mut T) -> Result<U, ExecutionError>
where T: RuleEngine<U> {
    for rule in rules {
        match engine.execute(&rule) {
            Err(err) => return Err(err),
            _ => {}
        }
    }

    Ok(engine.output())
}
