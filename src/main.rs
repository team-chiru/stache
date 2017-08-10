extern crate stachemu;
use stachemu::engines::Mustache;
use stachemu::command::Engine;
use stachemu::specs::mustache::{ MustachePool, TestPool };

extern crate serde_json;

fn main() {
    let base = String::from("specs/mustache/specs/");
    let path = base + "partials.yml";
    let mut pool = MustachePool::default();

    pool.path(&path);
    pool.name("Standalone Without Newline");

    let (template, partials, data) = pool.debug().unwrap();
    println!("{:?}", partials);

    let result = Mustache::render(template, partials, vec![data]).unwrap();
    println!("{:?}", result);
}
