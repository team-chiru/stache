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

use Template;
use error::RenderingError;

pub trait RuleEngine<R, Input, Output> {
    fn render(Template<R>, HashMap<String, Template<R>>, Vec<Input>) -> Result<Output, RenderingError>
    where Self: RuleEngine<R, Input, Output> + Sized + Debug, Input: Clone + Debug;
}
