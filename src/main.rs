extern crate serde_json;
use self::serde_json::Value;

extern crate stachemu;
use stachemu::engines::Mustache;
use stachemu::command::Engine;

use stachemu::specs::pool::{ Pool };

type MustachePool = Pool<Value, String>;

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
