extern crate mustache;
use mustache::{ Mustache, Test };

extern crate stache;
use stache::{ TemplateEngine };
use stache::testing::{ TestPool };

fn main() {
    let base = String::from("spec/specs/");
    let path = base + "partials.yml";
    let mut pool = Test::default();

    pool.path(&path);
    pool.name("Basic Behavior");

    let (template, partials, data) = pool.debug().unwrap();
    //println!("{:?}", template);

    let result = Mustache::render(template, partials, vec![data]).unwrap();
    println!("{}", result);
}
