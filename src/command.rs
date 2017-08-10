use rule::{ Template, Rule };
use std::fmt::Debug;
use error::ExecutionError;

#[derive(PartialEq, Debug, Clone)]
pub enum Command<Input, Output> {
    Skip(Rule),
    SliceOff(Rule, Vec<Input>, bool),
    Import(Rule),
    Write(Output),
    None
}

pub trait Engine<Input, Output> {
    fn decide(&Rule, &Input) -> Self;

    fn execute(self, &mut Template, &Vec<Input>) -> Result<Output, ExecutionError>;

    fn render(Template, Vec<Input>) -> Result<Output, ExecutionError>
    where Self: Engine<Input, Output> + Sized + Debug, Input: Clone + Debug;
}
