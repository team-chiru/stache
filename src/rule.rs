#[derive(PartialEq, Debug, Clone)]
pub enum Rule {
    Noop(String),
    Symbolic(String, String),
    Default(String)
}

pub type Template = Vec<Rule>;
