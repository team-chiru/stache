extern crate serde_json;
extern crate stache;
#[macro_use] extern crate stache_derive;

mod rule;
pub use self::rule::MustacheRule;

//FIXME mod engine;
