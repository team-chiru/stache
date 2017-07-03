extern crate stachemu;

macro_rules! get_spec {
    ($path:expr, $name:ident) => {{
        let path = String::from($path);
        let name = String::from(stringify!($name));

        if let Some(test) = Test::get(path, name) {
            test
        } else {
            panic!("Test not found")
        }
    }};
}

macro_rules! test_spec {
    ($path:expr => $name:ident) => {
        use stachemu::spec::Test;

        use stachemu::compile;
        use stachemu::process;

        use stachemu::engines::mustache::Builder;

        #[test]
        pub fn $name () {
            let test = get_spec!($path, $name);
            let rules = compile(test.template).unwrap();
            let mut builder = Builder::configure(test.data);

            let result = process::<Builder, String>(rules, &mut builder);
            if test.expected != result.unwrap() {
                panic!("Unimplemented spec")
            }
        }
    }
}

mod mustache {
    test_spec!("specs/mustache/specs/interpolation.yml" => no_interpolation);
}
