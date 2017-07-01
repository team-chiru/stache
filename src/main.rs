extern crate stachemu;
use stachemu::file;
extern crate serde_yaml;

use stachemu::spec::Spec;

fn main() {
    let spec_yaml = file::read("specs/mustache/specs/sections.yml").unwrap();

    let spec: Spec = serde_yaml::from_str(&spec_yaml).unwrap();
    for test in spec.tests {
        println!("{:?}", test.name);
    }
}
