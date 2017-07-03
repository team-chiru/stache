extern crate serde;
extern crate serde_yaml;
extern crate serde_json;

use self::serde_json::Value;
use file;

#[derive(Deserialize, Debug)]
pub struct Spec {
    pub tests: Vec<Test>
}

#[derive(Deserialize, Debug)]
pub struct Test {
    pub name: String,
    pub desc: String,
    pub data: Value,
    pub template: String,
    pub expected: String,
}

impl Test {
    fn format_ident(name: String) -> String {
        let idents: Vec<&str> = name.split('_').collect();
        let mut specs_idents: Vec<String> = vec!();

        for ident in idents {
            if ident.len() <= 0 {
                continue;
            }

            let (first, left) = ident.split_at(1);
            //let first = String::from(first);

            specs_idents.push(
                first.to_uppercase() + left
            )
        }

        specs_idents.join(" ")
    }

    pub fn get(path: String, name: String) -> Option<Test> {
        let yaml = file::read(&path).unwrap();
        let spec: Spec = serde_yaml::from_str(&yaml).unwrap();
        let test_name = Test::format_ident(name);
        let mut find = None;

        for test in spec.tests {
            if test_name == test.name {
                find = Some(test);
                break;
            }
        }

        find
    }
}
