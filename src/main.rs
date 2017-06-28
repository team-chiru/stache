extern crate stachemu;
use stachemu::compiler::compile;
use stachemu::file;
use stachemu::processor::process;
use stachemu::engines::mustache::Builder;

extern crate serde_json;
use serde_json::Value;

fn main() {
    let raw = file::read("samples/sample.mustache").unwrap();
    let rules = compile(raw).unwrap();

    let data =
r#"{
    "name": "NAME",
    "description": "DESCRIPTION",
    "url": {
        "name": "URL"
    }
}"#;

    let json: Value = serde_json::from_str(data).unwrap();
    let mut builder = Builder::configure(json);

    let result = process::<Builder, String>(rules, &mut builder);
    println!("{}", result.unwrap());
}
