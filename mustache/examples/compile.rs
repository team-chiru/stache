extern crate stache;
extern crate mustache;

use stache::{ file, compiling };

use mustache::MustacheRule;
use stache::compiling::{ Matcher, RuleMatching };

fn main() {
    println!("{:?}", Matcher::build(MustacheRule::configure_matching()));

    let sample = file::read("examples/sample.mustache").unwrap();
    println!("{:?}", compiling::compiles_template::<MustacheRule>(&sample));
}
