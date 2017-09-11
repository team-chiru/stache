extern crate serde_json;
use serde_json::{ Value, Map };

extern crate mustache;
use mustache::MustacheRule;

macro_rules! map {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(map!(@single $rest)),*]));

    ($($key:expr => $value:expr,)+) => { map!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let _cap = map!(@count $($key),*);
            let mut _map = ::serde_json::Map::with_capacity(_cap);
            $(
                _map.insert($key, $value);
            )*
            _map
        }
    };
}

fn main() {
    let serialized = r#"
{
    "inverted_section": ""
}
    "#;

    let mr: MustacheRule = serde_json::from_str(&serialized).unwrap();
    println!("{:?}", mr);

    let serialized2 = Value::Array(vec![
        Value::Object(map![
            "inverted_section".to_string() => Value::String(String::default())
        ]),
        Value::Object(map![
            "interpolation".to_string() => Value::String(String::default())
        ])
    ]);

    let mr2: Vec<MustacheRule> = serde_json::from_value(serialized2).unwrap();
    println!("{:?}", mr2);
}
