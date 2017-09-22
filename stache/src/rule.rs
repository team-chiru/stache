use expr::{ Delimiter, Command };

#[derive(PartialEq, Debug, Clone)]
pub enum DefaultRule {
    Symbolic(Delimiter, Command, String),
    Default(String),
    None
}

impl Default for DefaultRule {
    fn default() -> Self {
        DefaultRule::Default(String::default())
    }
}

pub trait Rule {
    fn is_comment(&self) -> bool { false }
    fn is_instruction(&self) -> bool { false }
    fn is_dotted(&self) -> bool { false }
}

impl Rule for DefaultRule {
    fn is_comment(&self) -> bool {
        if let DefaultRule::Symbolic(_, ref symbol, _) = *self {
            *symbol == String::from("!")
        } else {
            false
        }
    }

    fn is_instruction(&self) -> bool {
        if let DefaultRule::Symbolic(_, ref symbol, _) = *self {
            match symbol.as_ref() {
                "#" | "/" | "^" | ">" => true,
                _ => false
            }
        } else {
            false
        }
    }

    fn is_dotted(&self) -> bool {
        if let DefaultRule::Symbolic(_, _, ref key) = *self {
            key.contains(".")
        } else {
            false
        }
    }
}
