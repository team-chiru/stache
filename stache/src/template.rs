use rule::DefaultRule;
use expr::Description;
use serde_json::{ from_value, Value, Map };
use heck::CamelCase;
use serde;

macro_rules! map {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(map!(@single $rest)),*]));

    ($($key:expr => $value:expr,)+) => { map!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let _cap = map!(@count $($key),*);
            let mut _map = ::serde_json::Map::with_capacity(_cap);
            $(
                _map.insert($key, $value);
            )*
            _map
        }
    };
}

#[derive(PartialEq, Debug, Clone)]
pub struct Template<R> {
    rules: Vec<R>,
    buffer: i32,
    now: i32
}

impl Template<DefaultRule> {
    pub fn from_default<R>(self, descr: &Description) -> Template<R>
    where for<'de> R: serde::Deserialize<'de> + Clone + PartialEq {
        let mut array: Vec<Value> = vec![];

        for rule in self.rules {
            use self::DefaultRule::*;

            match rule {
                Symbolic(del, cmd, value) => {
                    if let Some(r) = descr.find(del, cmd) {
                        array.push(Value::Object(map![
                            r.name.to_camel_case() => Value::String(value.clone())
                        ]))
                    }
                },
                Default(ref value) => {
                    array.push(Value::Object(map![
                        String::from("Default") => Value::String(value.clone())
                    ]))
                },
                None => {}
            }
        }

        let rules: Vec<R> = from_value(Value::Array(array)).unwrap();

        Template::new(rules)
    }
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
