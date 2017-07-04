#![allow(dead_code)]

extern crate serde;
extern crate serde_yaml;
extern crate serde_json;

use self::serde_json::Value;

use file;
use compile;
use process;
use processor::TemplateEngine;

use super::spec::{ Test, Spec };

pub type MustacheSpec = Spec<Value, String>;
pub type MustacheTest = Test<Value, String>;

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

impl MustacheTest {
    pub fn process<T>(&self) -> String
    where T: TemplateEngine<Value, String> {
        let mut engine: T = T::configure(self.data.clone());
        let rules = compile(self.template.clone()).unwrap();
        process::<T, Value, String>(rules, &mut engine).unwrap()
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
