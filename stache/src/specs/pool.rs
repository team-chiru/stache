use serde_json::Value;

use std::collections::HashMap;
use std::fmt::Debug;

use super::spec::{ Test, Spec };
use rule::{ Template, Rule };
use compiler::{ compile, compile_partials };
use command::{ Engine };

pub trait TestPool<R, Input, Output> {
    fn path(&mut self, path: &str);
    fn name(&mut self, name: &str);
    fn process(&self) -> Option<Output>;
    fn debug(&self) -> Option<(Template<R>, HashMap<String, Template<R>>, Input)>;
}

#[derive(Default)]
pub struct Pool<Input, Output> {
    pub spec: Option<Spec<Test<Input, Output>>>,
    pub test: Option<Test<Input, Output>>
}

impl<R, Input, Output> Pool<Input, Output>
where R: Rule<R>,
      for<'de> Input: Default + Debug + Clone + serde::Deserialize<'de>,
      for<'de> Output: Debug + Clone + serde::Deserialize<'de> {
    pub fn path(&mut self, path: &str) {
        self.spec = Some(Spec::from_path(path));
    }

    pub fn name(&mut self, name: &str) {
        let mut test = None;
        if let Some(ref spec) = self.spec {
            test = Some(spec.get(name).clone());
        }

        self.test = test;
    }

    pub fn process<E>(&self) -> Option<Output>
    where E: Debug + Engine<R, Input, Output> {
        if let Some(ref test) = self.test {
            let data = vec![test.data.clone()];
            let rules = compile(test.template.clone()).unwrap();

            let mut partials = HashMap::new();
            if test.partials != Value::Null {
                partials = compile_partials(test.partials.clone()).unwrap();
            }

            Some(E::render(rules, partials, data).unwrap())
        } else {
            None
        }
    }

    pub fn debug(&self) -> Option<(Template<R>, HashMap<String, Template<R>>, Input)> {
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
