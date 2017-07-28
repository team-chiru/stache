#[derive(PartialEq, Debug, Clone)]
pub struct Symbol {
    symbol: String
}

impl<S> From<S> for Symbol where S: Into<String> {
    fn from(s: S) -> Self {
        Symbol { symbol: s.into() }
    }
}


impl Symbol {
    pub fn is_instruction(&self) -> bool {
        if self.symbol == "#" ||
            self.symbol == "/" ||
            self.symbol == "^" {
            true
        } else {
            false
        }
    }

    pub fn get(&self) -> &str {
        self.symbol.as_ref()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Rule {
    Noop(String),
    Symbolic(Symbol, String),
    Default(String)
}

impl Default for Rule {
    fn default() -> Self {
        Rule::Default(String::default())
    }
}

pub type Template = Vec<Rule>;
