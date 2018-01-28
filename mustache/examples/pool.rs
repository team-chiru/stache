extern crate mustache;
use mustache::{ Mustache, Test };

extern crate stache;
use stache::{ TemplateEngine };
use stache::testing::{ TestPool };

fn main() {
    let path = String::from("spec/specs/partials.yml");
    let mut pool = Test::default();

    pool.path(&path);
    pool.name("Standalone Indentation");

    let (template, partials, data) = pool.debug().unwrap();
    println!("{:?}", template);
    println!("{:?}", partials);

    let result = Mustache::render(template, partials, vec![data]).unwrap();
    
    println!("{}", result);
}
