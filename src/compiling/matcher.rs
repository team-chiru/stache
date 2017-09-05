use regex;
use expr;

#[derive(Debug, Clone)]
pub struct Matcher {
    tokens: Vec<regex::Regex>
}

impl Matcher {
    pub fn build<'e>(expr: expr::Expression<'e>) -> Result<Self, regex::Error> {
        let mut symbol_pattern = String::from("(?P<symbol>[");

        for symbol in expr.directives.clone() {
            symbol_pattern += regex::escape(symbol.into()).as_ref();
        }

        symbol_pattern += "]?)";

        let key_pattern = String::from("(?P<key>[") + expr.keySymbols + "]+)";
        let mut expressions: Vec<regex::Regex> = vec![];

        for ( open, close ) in expr.delimiters.clone() {
            let open_pattern = String::from("(?P<open>") + regex::escape(open).as_ref() + ")";
            let close_pattern = String::from("(?P<close>") + regex::escape(close).as_ref() + ")";

            let pattern = String::from("^") + &open_pattern + &symbol_pattern + &key_pattern + &close_pattern;

            match regex::Regex::new(pattern.as_ref()) {
                Ok(compiled) => expressions.push(compiled),
                Err(err) => return Err(err)
            }
        }

        Ok(Matcher { tokens: expressions })
    }

    pub fn captures<'c>(&self, input: &'c str) -> Option<regex::Captures<'c>> {
        for re in self.tokens.clone() {
            if let Some(capture) = re.captures(input) {
                return Some(capture)
            }
        }

        None
    }
}

pub trait RuleMatching {
    fn get_delimiters<'d>() -> Vec<expr::Delimiter<'d>>;
    fn get_directives<'s>() -> Vec<expr::Symbol<'s>>;
    fn get_default_keys<'k>() -> &'k str;

    fn configure_matching<'e>() -> expr::Expression<'e> {
        expr::Expression {
            delimiters: Self::get_delimiters(),
            directives: Self::get_directives(),
            keySymbols: Self::get_default_keys()
        }
    }
}
