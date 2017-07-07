extern crate stachemu;

use stachemu::compile;
use stachemu::engines::mustache::Builder;
use stachemu::engines::processor::{ Engine };

extern crate serde_json;

fn main() {
    //let mustache = file::read("samples/interpolation.mustache").unwrap();
    let mustache = String::from(r#""{{#a.b.c}}Here{{/a.b.c}}" == """#);
    let template = compile(mustache).unwrap();
    println!("{:?}", template);

    //let json = file::read("samples/interpolation.json").unwrap();
    let json = String::from(r#"{ "a": { "b": { "c": false } } }"#);
    let data = serde_json::from_str(&json).unwrap();

    println!("{}", Builder::process(template, data).unwrap());
}
