extern crate stachemu;

use stachemu::{ file, compiling };

use stachemu::implementations::mustache::MustacheRule;
use stachemu::compiling::{ Matcher, RuleMatching };

fn main() {
    println!("{:?}", Matcher::build(MustacheRule::configure_matching()));
    let sample = file::read("examples/sample.mustache").unwrap();

    compiling::compile();
}
