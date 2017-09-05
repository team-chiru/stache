use expr;

#[derive(PartialEq, Debug, Clone)]
pub enum DefaultRule<'r> {
    Iterator(expr::Delimiter<'r>, expr::Symbol<'r>),
    Symbolic(expr::Delimiter<'r>, expr::Symbol<'r>, String),
    Default(String),
    None
}

impl<'r> Default for DefaultRule<'r> {
    fn default() -> Self {
        DefaultRule::Default(String::default())
    }
}

pub trait Rule<'r> where Self: From<DefaultRule<'r>> + Into<DefaultRule<'r>> + Clone {
    fn is_comment(&self) -> bool {
        let default: DefaultRule = self.clone().into();

        if let DefaultRule::Symbolic(_, "!", _) = default {
            true
        } else {
            false
        }
    }

    fn is_instruction(&self) -> bool {
        let default: DefaultRule = self.clone().into();

        if let DefaultRule::Symbolic(_, symbol, _) = default {
            match symbol {
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

#[derive(PartialEq, Debug, Clone)]
pub struct Template<R> {
    rules: Vec<R>,
    buffer: i32,
    now: i32
}

impl<R> Template<R> where R: Clone + PartialEq {
    pub fn new(rules: Vec<R>) -> Self {
        Template {
            rules: rules,
            buffer: 0,
            now: -1
        }
    }

    pub fn get(&self, index: i32) -> Option<R> {
        self.rules.get(index as usize).map(|r| r.clone())
    }

    pub fn now(&self) -> i32 { self.now }

    pub fn find(&mut self, next: &R) -> Option<i32> {
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

    pub fn walk_until(&mut self, next: &R) -> Option<i32> {
        if let Some(index) = self.find(next) {
            self.buffer = index;
            Some(index)
        } else {
            None
        }
    }

    pub fn split_until(&mut self, next: &R) -> Option<Template<R>> {
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

impl<R> Iterator for Template<R> where R: Clone + PartialEq {
    type Item = R;

    fn next(&mut self) -> Option<R> {
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
