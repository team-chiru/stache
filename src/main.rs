extern crate stachemu;
use stachemu::file;
extern crate serde_json;

use stachemu::spec::Spec;

fn main() {
    let spec_yaml = file::read("specs/mustache/specs/sections.yml");

    let spec = Spec::new(spec_yaml.unwrap());
    for test in spec.tests {
        println!("{:?}", test.name);
    }
}
