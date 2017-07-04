#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]

extern crate stachemu;
use stachemu::file;

use stachemu::specs::mustache::{ MustacheSpec, MustacheTest };
use stachemu::engines::mustache::Builder;

describe! mustache {
    before_each {
        let base = String::from("specs/mustache/specs/");
    }

    describe! interpolation {
        before_each {
            let path = base + "interpolation.yml";
            let spec = MustacheSpec::from_path(&path);
        }

        it "No Interpolation" {
            let test = spec.get("No Interpolation");
            assert!(test.expected == test.process::<Builder>())
        }

        it "Basic Interpolation" {
            let test = spec.get("Basic Interpolation");
            assert!(test.expected == test.process::<Builder>())
        }

        it "HTML Escaping" {
            let test = spec.get("HTML Escaping");
            assert!(test.expected == test.process::<Builder>())
        }

        it "Triple Mustache" {
            let test = spec.get("Triple Mustache");
            assert!(test.expected == test.process::<Builder>())
        }

        it "Ampersand" {
            let test = spec.get("Ampersand");
            assert!(test.expected == test.process::<Builder>())
        }

        it "Basic Integer Interpolation" {
            let test = spec.get("Basic Integer Interpolation");
            assert!(test.expected == test.process::<Builder>())
        }

        it "Triple Mustache Integer Interpolation" {
            let test = spec.get("Triple Mustache Integer Interpolation");
            assert!(test.expected == test.process::<Builder>())
        }

        it "Ampersand Decimal Interpolation" {
            let test = spec.get("Ampersand Decimal Interpolation");
            assert!(test.expected == test.process::<Builder>())
        }
    }
}
