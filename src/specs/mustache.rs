extern crate serde;
extern crate serde_yaml;
extern crate serde_json;

use self::serde_json::Value;

use file;
use compile;
use engines::processor::{ Engine };

use super::spec::{ Test, Spec };

pub type MustacheTest = Test<Value, String>;
pub type MustacheSpec = Spec<MustacheTest>;

pub trait TestPool {
    fn path(&mut self, path: &str);
    fn name(&mut self, name: &str);
    fn process<E>(&self) -> Option<String> where E: Engine<Vec<Value>, String>;
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

    fn process<E>(&self) -> Option<String> where E: Engine<Vec<Value>, String> {
        if let Some(ref test) = self.test {
            let data = vec![test.data.clone()];
            let rules = compile(test.template.clone()).unwrap();
            let engine = E::new(data.clone());

            Some(engine.process(rules, data).unwrap())
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
