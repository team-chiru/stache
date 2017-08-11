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
            self.symbol == "^" ||
            self.symbol == ">" {
            true
        } else {
            false
        }
    }

    pub fn is_comment(&self) -> bool {
        if self.symbol == "!" {
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
    Noop(bool, String),
    Symbolic(bool, Symbol, String),
    Default(bool, String)
}

impl Default for Rule {
    fn default() -> Self {
        Rule::Default(false, String::default())
    }
}

impl Rule {
    pub fn is_written(&self) -> bool {
        use self::Rule::*;

        match *self {
            Noop(true, ..) | Symbolic(true, ..) | Default(true, ..) => {
                true
            },
            _ => false
        }
    }

    pub fn is_dotted(&self) -> bool {
        if let Rule::Symbolic(_, _, ref key) = *self {
            key.contains(".")
        } else {
            false
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Template {
    rules: Vec<Rule>,
    buffer: i32,
    now: i32
}

impl Template {
    pub fn new(rules: Vec<Rule>) -> Self {
        Template {
            rules: rules,
            buffer: 0,
            now: -1
        }
    }

    pub fn get(&self, index: i32) -> Option<Rule> {
        self.rules.get(index as usize).map(|r| r.clone())
    }

    pub fn find(&mut self, next: &Rule) -> Option<i32> {
        let mut nested_level = 0;
        let mut found = self.now;

        let old = match self.get(found) {
            Some(rule) => rule,
            None => return None
        };

        let start = (self.now + 1) as usize;
        let end = self.rules.len();

        for i in start..end {
            let current = match self.get(i as i32) {
                Some(rule) => rule,
                None => return None
            };

            if current == old {
                nested_level += 1;
            }

            if current == *next && nested_level <= 0 {
                found = i as i32;
                break;
            }

            if current == *next {
                nested_level -= 1;
            }
        }

        if found == self.now {
            None
        } else {
            Some(found)
        }
    }

    pub fn walk_until(&mut self, next: &Rule) -> Option<i32> {
        if let Some(index) = self.find(next) {
            self.buffer = index;
            Some(index)
        } else {
            None
        }
    }

    pub fn split_until(&mut self, next: &Rule) -> Option<Template> {
        if let Some(index) = self.find(next) {
            let now = self.now as usize;
            let tmpl = self.rules.clone();

            self.buffer = index;
            let index = index as usize;

            let (_, new) = tmpl.split_at(now + 1);
            let (section, _) = new.split_at(index - now - 1);

            Some(Template::new(section.to_vec()))
        } else {
            None
        }
    }
}

impl Iterator for Template {
    type Item = Rule;

    fn next(&mut self) -> Option<Rule> {
        if self.buffer != 0 {
            self.now = self.buffer;
            self.buffer = 0;
        } else {
            self.now += 1;
        }

        let next = self.now;
        self.get(next)
    }
}
