extern crate stachemu;
use stachemu::compiler::compile;
use stachemu::file;
use stachemu::processor::process;
use stachemu::engines::mustache::Builder;

extern crate serde_json;
use serde_json::Value;

use stachemu::specs::*;
use stachemu::generator::*;

fn main() {
/*    let raw = file::read("samples/sample.mustache").unwrap();
    let rules = compiler::compile(raw).unwrap();

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

    processor::process::<Builder, String>(rules, &mut builder).unwrap();

 */
    let path_yaml = "specs/mustache/specs/sections.yml";
    let path_json = "specs/mustache/specs/sections.json";


    let test_yaml: Specs = generate_from_yaml(String::from(path_yaml));
    let test_json: Specs = generate_from_json(String::from(path_json));


}
