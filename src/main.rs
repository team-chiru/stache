extern crate stachemu;
use stachemu::engines::Mustache;
use stachemu::command::Engine;
use stachemu::specs::mustache::{ MustachePool, TestPool };

extern crate serde_json;

fn main() {
    let base = String::from("specs/mustache/specs/");
    let path = base + "sections.yml";
    let mut pool = MustachePool::default();

    pool.path(&path);
    pool.name("Implicit Iterator - Array");

    let (template, data) = pool.debug().unwrap();
    let result = Mustache::process_all(template, vec![data]).unwrap();
    println!("{}", result);
}
