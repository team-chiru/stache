#![warn(unused_imports)]
#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]

extern crate serde_json;
use self::serde_json::Value;

extern crate stache;
use stache::engines::{ Stachemu };
use stache::specs::pool::{ Pool };

type StachemuPool = Pool<String, Value>;

describe! stachemu_tests {
    before_each {
        let base = String::from("specs/stachemu/");
        let mut pool = StachemuPool::default();
    }

    describe! interpolation {
        before_each {
            let path = base + "interpolation.yml";
            pool.path(&path);
        }

        describe! simple {
            it "no" { pool.name("No Interpolation"); }
            it "null" { pool.name("No Matching"); }
            it "basic" { pool.name("Basic Interpolation"); }
        }

        describe! unclosed {
            it "basic" { pool.name("Unclosed Interpolation"); }
            it "still null" { pool.name("Still No Matching"); }
        }

        after_each {
            let result = pool.process::<Stachemu>().unwrap();
            let expected = pool.test.unwrap().expected;

            println!("expected: \n{:?}", expected);
            println!("result: \n{:?}", result);
            assert!(expected == result)
        }
    }
}
