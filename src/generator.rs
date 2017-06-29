extern crate serde_derive;
extern crate serde_yaml;
extern crate serde_json;

use file::*;
use specs::Specs;

struct GeneratorX {}

pub fn generate_from_yaml(path: String) -> Specs {
    let f = read(&path).unwrap();

    println!("{:?}",f);

    Specs{
        name: String::from(" "),
        desc: String::from(" "),
        data: String::from(" "),
        template: String::from(" "),
        expected: String::from(" ")
    }
}

pub fn generate_from_json(path: String) -> Specs {
    let f = read(&path).unwrap();

    println!("{:?}",f);

    Specs{
        name: String::from(" "),
        desc: String::from(" "),
        data: String::from(" "),
        template: String::from(" "),
        expected: String::from(" ")
    }
}
