extern crate stachemu;
extern crate serde_yaml;

extern crate serde_json;
use serde_json::Value;

macro_rules! get_spec {
    ($path:expr, $name:ident) => {
        let path = String::from($path);
        let name = String::from(stringify!($name));

        if let Some(test) = Test::get(path, name) {
            test
        } else {
            panic!("Test not found")
        }
    }
}

macro_rules! make_spec {
    ($path:expr => $name:ident) => {
        use stachemu::spec::Test;

        #[test]
        pub fn $name () {
            let test = get_spec!($path, $name);
            let data = test.data;

        }
    }
}

mod test {
    make_spec!("specs/mustache/specs/interpolation.yml" => no_interpolation);
}

fn main() {}
