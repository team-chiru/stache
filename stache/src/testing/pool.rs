use std::fmt::Debug;
use std::marker::PhantomData;

use super::spec::{ Test, Spec };
use { Template, Partials, TemplateCompiler, TemplateEngine };

use serde;

pub trait TestPool<R, Input, Output> {
    fn path(&mut self, path: &str);
    fn name(&mut self, name: &str);
    fn process(&self) -> Option<Output>;
    fn debug(&self) -> Option<(Template<R>, Partials<R>, Input)>;
}

#[derive(Default)]
pub struct Pool<R, Input, Output> {
    pub spec: Option<Spec<Test<Input, Output>>>,
    pub test: Option<Test<Input, Output>>,
    pub pool_type: PhantomData<R>
}

impl<R, Input, Output> TestPool<R, Input, Output> for Pool<R, Input, Output> where
for<'de> R: serde::Deserialize<'de> + Clone + PartialEq + TemplateCompiler + TemplateEngine<R, Input, Output> + Debug,
for<'de> Input: Default + Debug + Clone + serde::Deserialize<'de>,
for<'de> Output: Debug + Clone + serde::Deserialize<'de> {
    fn path(&mut self, path: &str) {
        self.spec = Some(Spec::from_path(path));
    }

    fn name(&mut self, name: &str) {
        let mut test = None;
        if let Some(ref spec) = self.spec {
            test = Some(spec.get(name).clone());
        }

        self.test = test;
    }

    fn process(&self) -> Option<Output> {
        if let Some(ref test) = self.test {
            let data = vec![test.data.clone()];

            let (tmpl, partials) = R::compiles_all(test.template.clone(), test.partials.clone()).unwrap();

            Some(R::render(tmpl, partials, data).unwrap())
        } else {
            None
        }
    }

    fn debug(&self) -> Option<(Template<R>, Partials<R>, Input)> {
        if let Some(ref test) = self.test {
            let (tmpl, partials) = R::compiles_all(test.template.clone(), test.partials.clone()).unwrap();
            //println!("{:?}", partials);
            Some((tmpl, partials, test.data.clone()))
        } else {
            None
        }
    }
}
