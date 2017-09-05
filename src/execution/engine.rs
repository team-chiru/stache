use std::fmt::Debug;
use std::collections::HashMap;

/*
#[derive(PartialEq, Debug, Clone)]
pub enum Command<Input, Output> {
    Skip(R),
    Extract(R, Vec<Input>, bool),
    Import(String),
    Write(Output),
    None
}
*/

use rule::Template;
use execution::ExecutionError;

pub trait Engine<R, Input, Output> {
    fn render(Template<R>, HashMap<String, Template<R>>, Vec<Input>) -> Result<Output, ExecutionError>
    where Self: Engine<R, Input, Output> + Sized + Debug, Input: Clone + Debug;
}
