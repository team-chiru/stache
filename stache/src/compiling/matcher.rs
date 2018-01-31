use regex;
use expr;
use expr::Delimiter;

#[derive(Debug, Clone)]
struct Pattern {
    name: String,
    search_expr: String,
    repeat_sym: Option<String>
}

impl Pattern {
    fn new(name: String, repeat: Option<String>) -> Self {
        Pattern {
            name: name,
            search_expr: String::default(),
            repeat_sym: repeat
        }
    }

    fn add_expr(&mut self, expr: String) {
        self.search_expr.push_str(regex::escape(expr.as_ref()).as_ref());
    }

    fn add_escaped_expr(&mut self, expr: String) {
        self.search_expr.push_str(expr.as_ref());
    }

    fn build_expr(&self) -> Option<String> {
        if self.search_expr.is_empty() {
            None
        } else {
            let open = String::from("(?P<") + &self.name + ">";
            let close = String::from(")");

            if self.repeat_sym.is_none() {
                Some(open + &self.search_expr + &close)
            } else {
                Some(
                    open + "[" + &self.search_expr + "]" + &self.repeat_sym.clone().unwrap() + &close
                )
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Matcher {
    constraints: Vec<regex::Regex>
}


impl Matcher {
    pub fn build(expr: &expr::Expression) -> Result<Self, regex::Error> {
        let mut symbol_pattern = Pattern::new(String::from("symbol"), Some(String::from("?")));
        let mut key_pattern = Pattern::new(String::from("key"), Some(String::from("+")));
        let mut expressions: Vec<regex::Regex> = vec![];

        for symbol in expr.commands.clone() {
            symbol_pattern.add_expr(symbol);
        }

        key_pattern.add_escaped_expr(expr.key_regex.clone());

        for Delimiter { open, close } in expr.delimiters.clone() {
            let mut open_pattern = Pattern::new(String::from("open"), None);
            let mut close_pattern = Pattern::new(String::from("close"), None);
            
            open_pattern.add_expr(open);
            close_pattern.add_expr(close);

            let mut pattern = String::from("^");

            if let Some(open_expr) = open_pattern.build_expr() {
                pattern.push_str(&open_expr);
            }

            if let Some(symbol_expr) = symbol_pattern.build_expr() {
                pattern.push_str(&symbol_expr);
            }

            if let Some(key_expr) = key_pattern.build_expr() {
                pattern.push_str(&key_expr);
            }

            if let Some(close_expr) = close_pattern.build_expr() {
                pattern.push_str(&close_expr);
            }

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
