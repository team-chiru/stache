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

pub trait Rule<'r> where Self: From<DefaultRule> + Into<DefaultRule> + Clone {
    fn is_comment(&self) -> bool {
        let default: DefaultRule = self.clone().into();

        if let DefaultRule::Symbolic(_, ref symbol, _) = default {
            *symbol == String::from("!")
        } else {
            false
        }
    }

    fn is_instruction(&self) -> bool {
        let default: DefaultRule = self.clone().into();

        if let DefaultRule::Symbolic(_, symbol, _) = default {
            match symbol.as_ref() {
                "#" | "/" | "^" | ">" => true,
                _ => false
            }
        } else {
            false
        }
    }

    fn is_dotted(&self) -> bool {
        let default: DefaultRule = self.clone().into();

        if let DefaultRule::Symbolic(_, _, ref key) = default {
            key.contains(".")
        } else {
            false
        }
    }
}
