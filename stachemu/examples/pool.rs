extern crate stachemu;
use stachemu::{ Stachemu, Test };

extern crate stache;
use stache::{ TemplateEngine, TemplateCompiler };
use stache::testing::{ TestPool };

fn main() {
    let path = String::from("spec/interpolation.yml");
    let mut pool = Test::default();

    pool.path(&path);
    pool.name("Dotted Names - Complex Interpolation");

    let (template, partials, data) = pool.debug().unwrap();

    let raw_template = String::from("I {{method.how}} {{method.what}}!");
    let template = Stachemu::compiles_template(raw_template).unwrap();
    let data = String::from("I say hello!");
    
    let result = Stachemu::render_once(template, vec![data]);
    println!("{:?}", result);
}