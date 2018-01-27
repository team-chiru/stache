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

use { Template, Partials };
use error::RenderingError;

pub trait TemplateEngine<R, Input, Output> {
    fn render(Template<R>, Partials<R>, Vec<Input>) -> Result<Output, RenderingError>
    where Self: TemplateEngine<R, Input, Output> + Sized + Debug, Input: Clone + Debug;
}
