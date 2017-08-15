extern crate serde_json;
use self::serde_json::Value;

extern crate stachemu;
use stachemu::engines::Stachemu;
use stachemu::command::Engine;

use stachemu::specs::pool::{ Pool };


type StachemuPool = Pool<String, Value>;

fn main() {
    let base = String::from("specs/stachemu/");
    let path = base + "interpolation.yml";
    let mut pool = StachemuPool::default();

    pool.path(&path);
    pool.name("Basic Interpolation");

    let (template, partials, data) = pool.debug().unwrap();
    let result = Stachemu::render(template, partials, vec![data]).unwrap();
    println!("{:?}", result);
}
