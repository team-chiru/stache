extern crate serde;
extern crate serde_yaml;
extern crate serde_json;

use self::serde_json::Value;
use std::collections::HashMap;

use file;
use compiler::{ compile, compile_partials };

use rule::Template;

use super::spec::{ Test, Spec };
use command::Engine;
use engines::Mustache;

pub type MustacheTest = Test<Value, String>;
pub type MustacheSpec = Spec<MustacheTest>;

pub trait TestPool {
    fn path(&mut self, path: &str);
    fn name(&mut self, name: &str);
    fn process(&self) -> Option<String>;
    fn debug(&self) -> Option<(Template, HashMap<String, Template>, Value)>;
}

#[derive(Default)]
pub struct MustachePool {
    pub spec: Option<MustacheSpec>,
    pub test: Option<MustacheTest>
}

impl TestPool for MustachePool {
    fn path(&mut self, path: &str) {
        let yaml = file::read(path).unwrap();

        self.spec = match serde_yaml::from_str(&yaml) {
            Ok(spec) => spec,
            Err(_) => None
        }
    }

    fn name(&mut self, name: &str) {
        let mut test: Option<MustacheTest> = None;
        if let Some(ref spec) = self.spec {
            test = Some(spec.get(name));
        }

        self.test = test;
    }

    fn process(&self) -> Option<String> {
        if let Some(ref test) = self.test {
            let data = vec![test.data.clone()];
            let rules = compile(test.template.clone()).unwrap();

            let mut partials = HashMap::new();
            if test.partials != Value::Null {
                partials = compile_partials(test.partials.clone()).unwrap();
            }

            Some(Mustache::render(rules, partials, data).unwrap())
        } else {
            None
        }
    }

    fn debug(&self) -> Option<(Template, HashMap<String, Template>, Value)> {
        if let Some(ref test) = self.test {
            let template = compile(test.template.clone()).unwrap();

            let mut partials = HashMap::new();
            if test.partials != Value::Null {
                partials = compile_partials(test.partials.clone()).unwrap();
            }

            Some((template, partials, test.data.clone()))
        } else {
            None
        }
    }
}

impl MustacheSpec {
    pub fn from_path(path: &String) -> Self {
        let yaml = file::read(path).unwrap();
        match serde_yaml::from_str(&yaml) {
            Ok(spec) => spec,
            Err(err) => panic!("{:?}", err)
        }
    }

    pub fn get(&self, name: &str) -> Test<Value, String> {
        let test_name = format_ident(&name.to_string());
        let ref tests = self.tests;
        let mut find = None;

        for test in tests {
            if test_name == test.name {
                find = Some(test.clone());
                break;
            }
        }

        if let Some(test) = find {
            test
        } else {
            panic!("Test *{:?}* not found", name);
        }
    }
}



fn format_ident(name: &String) -> String {
    let idents: Vec<&str> = name.split('_').collect();

    if idents.len() == 1 {
        return String::from(idents[0]);
    }

    let mut specs_idents: Vec<String> = vec!();

    for ident in idents {
        if ident.len() <= 0 {
            continue;
        }

        let (first, left) = ident.split_at(1);
        //let first = String::from(first);

        specs_idents.push(
            first.to_uppercase() + &left.to_lowercase()
        )
    }

    specs_idents.join(" ")
}
