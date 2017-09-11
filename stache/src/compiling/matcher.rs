use regex;
use expr;
use expr::Delimiter;

#[derive(Debug, Clone)]
pub struct Matcher {
    constraints: Vec<regex::Regex>
}

impl Matcher {
    pub fn build(expr: &expr::Expression) -> Result<Self, regex::Error> {
        let mut symbol_pattern = String::from("(?P<symbol>[");

        for symbol in expr.commands.clone() {
            symbol_pattern += regex::escape(symbol.as_ref()).as_ref();
        }

        symbol_pattern += "]?)";

        let key_pattern = String::from("(?P<key>[") + expr.key_regex.as_ref() + "]+)";
        let mut expressions: Vec<regex::Regex> = vec![];

        for Delimiter { open, close } in expr.delimiters.clone() {
            let open_pattern = String::from("(?P<open>") + regex::escape(open.as_ref()).as_ref() + ")";
            let close_pattern = String::from("(?P<close>") + regex::escape(close.as_ref()).as_ref() + ")";

            let pattern = String::from("^") + &open_pattern + &symbol_pattern + &key_pattern + &close_pattern;

            match regex::Regex::new(pattern.as_ref()) {
                Ok(compiled) => expressions.push(compiled),
                Err(err) => return Err(err)
            }
        }

        Ok(Matcher { constraints: expressions })
    }

    pub fn captures<'r>(&'r self, input: &'r String) -> Option<regex::Captures<'r>> {
        for re in self.constraints.clone() {
            if let Some(capture) = re.captures(input) {
                return Some(capture);
            }
        }

        None
    }
}
