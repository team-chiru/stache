extern crate stachemu;
use stachemu::compiler;
use stachemu::file;
use stachemu::processor;
use stachemu::engines::mustache::Builder;

extern crate serde_json;
use serde_json::Value;

fn main() {
    let raw = file::read("samples/sample.mustache").unwrap();
    let rules = compiler::compile(raw).unwrap();

    let data =
r#"{
    "name": "John Doe",
    "age": 43,
    "phones": [
        "+44 1234567",
        "+44 2345678"
    ]
}"#;

    let json: Value = serde_json::from_str(data).unwrap();
    let mut builder = Builder::configure(json);

    processor::process::<Builder, String>(rules, &mut builder).unwrap();
}
