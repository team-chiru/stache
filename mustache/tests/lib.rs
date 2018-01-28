#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]
#![allow(unused_imports)]

extern crate stache;
extern crate mustache;

use mustache::{ Mustache, Test };
use stache::testing::{ TestPool };

describe! mustache_tests {
    before_each {
        let base = String::from("spec/specs/");
        let mut pool = Test::default();
    }

    describe! interpolation {
        before_each {
            let path = base + "interpolation.yml";
            pool.path(&path);
        }

        describe! simple {
            it "no" { pool.name("No Interpolation"); }
            it "basic" { pool.name("Basic Interpolation"); }
            it "escaping" { pool.name("HTML Escaping"); }
            it "triple" { pool.name("Triple Mustache"); }
            it "ampersand" { pool.name("Ampersand"); }
        }

        describe! integer {
            it "basic" { pool.name("Basic Integer Interpolation"); }
            it "triple" { pool.name("Triple Mustache Integer Interpolation"); }
            it "ampersand" { pool.name("Ampersand Integer Interpolation"); }
        }

        describe! decimal {
            it "basic" { pool.name("Basic Decimal Interpolation"); }
            it "triple" { pool.name("Triple Mustache Decimal Interpolation"); }
            it "ampersand" { pool.name("Ampersand Decimal Interpolation"); }
        }

        describe! miss {
            it "basic" { pool.name("Basic Context Miss Interpolation"); }
            it "triple" { pool.name("Triple Mustache Context Miss Interpolation"); }
            it "ampersand" { pool.name("Ampersand Context Miss Interpolation"); }
        }

        describe! dotted_names {
            it "basic" { pool.name("Dotted Names - Basic Interpolation"); }
            it "triple" { pool.name("Dotted Names - Triple Mustache Interpolation"); }
            it "ampersand" { pool.name("Dotted Names - Ampersand Interpolation"); }
            it "arbitrary depth" { pool.name("Dotted Names - Arbitrary Depth"); }
            it "broken" { pool.name("Dotted Names - Broken Chains"); }
            it "broken resolution" { pool.name("Dotted Names - Broken Chain Resolution"); }
            it "initial resolution" { pool.name("Dotted Names - Initial Resolution"); }
            it "precedence" { pool.name("Dotted Names - Context Precedence"); }
        }

         describe! whitespace_sensivity {
            it "basic whitespace" { pool.name("Interpolation - Surrounding Whitespace"); }
            it "triple whitespace" { pool.name("Triple Mustache - Surrounding Whitespace"); }
            it "ampersand whitespace" { pool.name("Ampersand - Surrounding Whitespace"); }
            it "basic standalone" { pool.name("Interpolation - Standalone"); }
            it "triple standalone" { pool.name("Triple Mustache - Standalone"); }
            it "ampersand standalone" { pool.name("Ampersand - Standalone"); }
         }

        describe! whitespace_insensitivity {
            it "basic" { pool.name("Interpolation With Padding"); }
            it "triple" { pool.name("Triple Mustache With Padding"); }
            it "ampersand" { pool.name("Ampersand With Padding"); }
        }
    }

    describe! sections {
        before_each {
            let path = base + "sections.yml";
            pool.path(&path);
        }

        describe! simple {
            it "truthy" { pool.name("Truthy"); }
            it "falsey" { pool.name("Falsey"); }
            it "doubled" { pool.name("Doubled"); }
            it "nested truthy" { pool.name("Nested (Truthy)"); }
            it "nested falsey" { pool.name("Nested (Falsey)"); }
        }

        describe! list {
            it "basic" { pool.name("List"); }
            it "empty" { pool.name("Empty List"); }
        }

        describe! context {
            it "basic" { pool.name("Context"); }
            it "nested" { pool.name("Deeply Nested Contexts"); }
            it "misses" { pool.name("Context Misses"); }
        }

        describe! iterator {
            it "string" { pool.name("Implicit Iterator - String"); }
            it "integer" { pool.name("Implicit Iterator - Integer"); }
            it "decimal" { pool.name("Implicit Iterator - Decimal"); }
            it "array" { pool.name("Implicit Iterator - Array"); }
        }

        describe! dotted_names {
            it "truthy" { pool.name("Dotted Names - Truthy"); }
            it "falsey" { pool.name("Dotted Names - Falsey"); }
            it "broken" { pool.name("Dotted Names - Broken Chains"); }
        }

        describe! whitespace_sensivity {
            it "surrounding" { pool.name("Surrounding Whitespace"); }
            // FIXME #4 it "internal" { pool.name("Internal Whitespace"); }
            // FIXME #4 it "indented inline" { pool.name("Indented Inline Sections"); }
            it "standalone" { pool.name("Standalone Lines"); }
            it "intended standalone" { pool.name("Indented Standalone Lines"); }
            // FIXME #4 it "line endings" { pool.name("Standalone Line Endings"); }
            // FIXME #4 it "without previous line" { pool.name("Standalone Without Previous Line"); }
            // FIXME #4 it "without newline" { pool.name("Standalone Without Newline"); }
        }

        describe! whitespace_insensitivity {
            it "padding" { pool.name("Padding"); }
        }
    }

    describe! inverted {
        before_each {
            let path = base + "inverted.yml";
            pool.path(&path);
        }

        describe! simple {
            it "truthy" { pool.name("Truthy"); }
            it "falsey" { pool.name("Falsey"); }
            it "doubled" { pool.name("Doubled"); }
            it "nested truthy" { pool.name("Nested (Truthy)"); }
            it "nested falsey" { pool.name("Nested (Falsey)"); }
        }

        describe! list {
            it "basic" { pool.name("List"); }
            it "empty" { pool.name("Empty List"); }
        }

        describe! context {
            it "basic" { pool.name("Context"); }
            it "misses" { pool.name("Context Misses"); }
        }

        describe! dotted_names {
            it "truthy" { pool.name("Dotted Names - Truthy"); }
            it "falsey" { pool.name("Dotted Names - Falsey"); }
            it "broken" { pool.name("Dotted Names - Broken Chains"); }
        }

        describe! whitespace_sensivity {
            it "surrounding" { pool.name("Surrounding Whitespace"); }
            // FIXME #4 it "internal" { pool.name("Internal Whitespace"); }
            // FIXME #4 it "indented inline" { pool.name("Indented Inline Sections"); }
            it "standalone" { pool.name("Standalone Lines"); }
            it "intended standalone" { pool.name("Standalone Indented Lines"); }
            // FIXME #4 it "line endings" { pool.name("Standalone Line Endings"); }
            // FIXME #4 it "without previous line" { pool.name("Standalone Without Previous Line"); }
            // FIXME #4 it "without newline" { pool.name("Standalone Without Newline"); }
        }

        describe! whitespace_insensitivity {
            it "padding" { pool.name("Padding"); }
        }
    }

    describe! comments {
        before_each {
            let path = base + "comments.yml";
            pool.path(&path);
        }

        describe! simple {
            it "inline" { pool.name("Inline"); }
            it "multiline" { pool.name("Multiline"); }
        }

        describe! whitespace_sensivity {
            it "surrounding" { pool.name("Surrounding Whitespace"); }
            it "standalone" { pool.name("Standalone"); }
            it "intended standalone" { pool.name("Indented Standalone"); }
            // FIXME #4 it "line endings" { pool.name("Standalone Line Endings"); }
            // FIXME #4 it "without previous line" { pool.name("Standalone Without Previous Line"); }
            // FIXME #4 it "without newline" { pool.name("Standalone Without Newline"); }
            it "multiline standalone" { pool.name("Multiline Standalone"); }
            // FIXME #4 it "indented inline" { pool.name("Indented Inline"); }
        }
    }

    describe! partial {
        before_each {
            let path = base + "partials.yml";
            pool.path(&path);
        }

        describe! simple {
            it "basic behaviour" { pool.name("Basic Behavior"); }
            it "failed lookup" { pool.name("Failed Lookup"); }
            it "context" { pool.name("Context"); }
            it "recursion" { pool.name("Recursion"); }
        }

        describe! whitespace_sensivity {
            it "surrounding whitespace" { pool.name("Surrounding Whitespace"); }
            // FIXME #4 it "inline indentation" { pool.name("Inline Indentation"); }
            it "line endings" { pool.name("Standalone Line Endings"); }
            // FIXME #4 it "without previous line" { pool.name("Standalone Without Previous Line"); }
            // FIXME #4 it "without newline" { pool.name("Standalone Without Newline"); }
            // FIXME #4 it "standalone indentation" { pool.name("Standalone Indentation"); }
        }

        describe! whitespace_insensitivity {
            it "padding whitespace" { pool.name("Padding Whitespace"); }
        }
    }

    after_each {
        let result = pool.process().unwrap();
        let expected = pool.test.unwrap().expected;

        println!("expected: \n{:?}", expected);
        println!("result: \n{:?}", result);
        assert!(expected == result)
    }
}
