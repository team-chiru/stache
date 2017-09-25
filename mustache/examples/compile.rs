extern crate mustache;
use mustache::Mustache;

extern crate stache;
use stache::{ file, TemplateCompiler };

fn main() {
    let sample = file::read("examples/sample.mustache").unwrap();

    println!("{:?}", Mustache::compiles_template(sample));
}
