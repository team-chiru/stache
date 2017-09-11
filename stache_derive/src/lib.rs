mod directive;
use directive::Directive;
use directive::{ Rules };

#[macro_use] extern crate serde_derive;
extern crate serde;

#[macro_use] extern crate proc_macro;
use proc_macro::TokenStream;

#[macro_use] extern crate quote;
use quote::Ident;

extern crate syn;
use syn::{ Attribute, Lit, MetaItem };

extern crate stache;
use stache::file;

extern crate toml;

#[proc_macro_derive(RuleMatcher, attributes(registry, delimiter, command, iterator))]
pub fn derive_rules(input: TokenStream) -> TokenStream {
    let source = input.to_string();

    // Parse the string representation into a syntax tree
    let ast = syn::parse_derive_input(&source).unwrap();
    let name = Ident::new(ast.ident.as_ref());

    let directives: Vec<Rules> = ast.attrs.iter().filter_map(|attr| {
        use self::MetaItem::*;
        match attr.value {
            NameValue(ref ident, Lit::Str(ref path, _)) if ident.as_ref() == "registry" => {
                Some(toml::from_str(&file::read(path).unwrap()).unwrap())
            },
            _ => None
        }
    }).collect();

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        // ...
    };

    // Parse back to a token stream and return it
    expanded.parse().unwrap()
}
