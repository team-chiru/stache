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

    processor::process::<Builder, Value>(rules).unwrap();
}
