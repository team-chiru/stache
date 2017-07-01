extern crate serde;
extern crate serde_yaml;
extern crate serde_json;

use self::serde_json::Value;

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
}
