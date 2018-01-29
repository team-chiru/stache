extern crate stachemu;
use stachemu::{ Stachemu, Test };

extern crate stache;
use stache::{ TemplateEngine };
use stache::testing::{ TestPool };

fn main() {
    let path = String::from("spec/interpolation.yml");
    let mut pool = Test::default();

    pool.path(&path);
    pool.name("Dotted Names - Complex Interpolation");

    let (template, partials, data) = pool.debug().unwrap();
    println!("{:?}", template);
    println!("{:?}", partials);
    
    let result = Stachemu::render(template, partials, vec![data]).unwrap();
    println!("{:?}", result);
}