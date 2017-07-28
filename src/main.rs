extern crate stachemu;

use stachemu::compile;
use stachemu::engines::mustache::Builder;
use stachemu::engines::processor::{ Engine };

extern crate serde_json;

fn main() {
    //let mustache = file::read("samples/interpolation.mustache").unwrap();
    let mustache = String::from(r#"
{{#bool}}
* first
{{/bool}}
* {{two}}
{{#bool}}
* third
{{/bool}}
"#);

    let template = compile(mustache).unwrap();
    println!("{:?}", template);

    //let json = file::read("samples/interpolation.json").unwrap();
    let json = String::from(r#"{ "bool": true, "two": "second" }"#);
    let data: serde_json::Value = serde_json::from_str(&json).unwrap();

    let builder = Builder::new(vec![data.clone()]);
    println!("{}", builder.process(template, vec![data]).unwrap());
}
