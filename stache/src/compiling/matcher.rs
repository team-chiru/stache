use regex;
use expr;

#[derive(Debug, Clone)]
pub struct Matcher {
    tokens: Vec<regex::Regex>
}

impl Matcher {
    pub fn build(expr: expr::Expression) -> Result<Self, regex::Error> {
        let mut symbol_pattern = String::from("(?P<symbol>[");

        for symbol in expr.directives.clone() {
            symbol_pattern += regex::escape(symbol.as_ref()).as_ref();
        }

        symbol_pattern += "]?)";

        let key_pattern = String::from("(?P<key>[") + expr.key_regex.as_ref() + "]+)";
        let mut expressions: Vec<regex::Regex> = vec![];

        for ( open, close ) in expr.delimiters.clone() {
            let open_pattern = String::from("(?P<open>") + regex::escape(open.as_ref()).as_ref() + ")";
            let close_pattern = String::from("(?P<close>") + regex::escape(close.as_ref()).as_ref() + ")";

            let pattern = String::from("^") + &open_pattern + &symbol_pattern + &key_pattern + &close_pattern;

            match regex::Regex::new(pattern.as_ref()) {
                Ok(compiled) => expressions.push(compiled),
                Err(err) => return Err(err)
            }
        }

        Ok(Matcher { tokens: expressions })
    }

    pub fn captures<'r>(&'r self, input: &'r String) -> Option<regex::Captures<'r>> {
        for re in self.tokens.clone() {
            if let Some(capture) = re.captures(input) {
                return Some(capture);
            }
        }

        None
    }
}

pub trait RuleMatching {
    fn get_delimiters() -> Vec<expr::Delimiter>;
    fn get_directives() -> Vec<expr::Symbol>;
    fn get_default_keys() -> String;

    fn configure_matching<'e>() -> expr::Expression {
        expr::Expression {
            delimiters: Self::get_delimiters(),
            directives: Self::get_directives(),
            key_regex: Self::get_default_keys()
        }
    }
}
