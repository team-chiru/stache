extern crate stachemu;

use stachemu::compile;
use stachemu::file;
use stachemu::engines::mustache::Builder;
use stachemu::engines::processor::{ TemplateEngine, Engine };

extern crate serde_json;

fn main() {
    let mustache = file::read("samples/interpolation.mustache").unwrap();
    let template = compile(mustache).unwrap();

    let json = file::read("samples/interpolation.json").unwrap();
    let data = serde_json::from_str(&json).unwrap();
    println!("{:?}", data);

    let mut builder = Builder::configure(data);
    println!("{}", builder.process(template).unwrap());
}
