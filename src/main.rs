extern crate stachemu;
use stachemu::engines::mustache::Builder;

use stachemu::specs::mustache::{ MustachePool, TestPool };

extern crate serde_json;

fn main() {
    let base = String::from("specs/mustache/specs/");
    let path = base + "comments.yml";
    let mut pool = MustachePool::default();

    pool.path(&path);

    pool.name("Multiline");
    let result = pool.process::<Builder>().unwrap();
    println!("result: \n{:?}", result);
}
