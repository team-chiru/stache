extern crate stachemu;
use stachemu::{ Stachemu };

extern crate stache;
use stache::{ TemplateEngine, TemplateCompiler };

fn main() {
    let template = Stachemu::compiles_template(
        String::from("I {{method.how}} {{method.what}}!")
    ).unwrap();

    let data = String::from("I say hello!");
    
    let result = Stachemu::render_once(template, vec![data]);
    println!("{:?}", result);
}