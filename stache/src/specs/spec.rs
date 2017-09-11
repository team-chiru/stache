extern crate serde;
extern crate serde_yaml;
extern crate serde_json;

use self::serde_json::Value;
use file;

#[derive(Deserialize, Debug)]
pub struct Spec<Test> {
    pub tests: Vec<Test>
}

#[derive(Clone, Deserialize, Debug)]
pub struct Test<Input, Output> {
    pub name: String,
    pub desc: String,
    pub data: Input,
    pub template: String,
    pub expected: Output,

    #[serde(default)]
    pub partials: Value
}

impl<Input, Output> Spec<Test<Input, Output>>
where for<'de> Input: Default + serde::Deserialize<'de>,
      for<'de> Output: serde::Deserialize<'de> {
    pub fn from_path(path: &str) -> Self {
        let yaml = file::read(path).unwrap();
        match serde_yaml::from_str(&yaml) {
            Ok(spec) => spec,
            Err(err) => panic!("{:?}", err)
        }
    }

    pub fn get(&self, name: &str) -> &Test<Input, Output> {
        let test_name = format_ident(&name.to_string());
        let ref tests = self.tests;
        let mut find = None;

        for test in tests {
            if test_name == test.name {
                find = Some(test.clone());
                break;
            }
        };

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
