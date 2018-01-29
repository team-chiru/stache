use stache::{ Template };
use serde_json::{ Value, Map };

use Stachemu;

type Model = Template<Stachemu>;

#[derive(Debug, Clone)]
pub struct ObjectWriter {
    pub buffer: Value,
    pub is_written: bool,
    pub template: Model
}

impl ObjectWriter {
    pub fn new(tmpl: Model) -> Self {
        Self {
            buffer: Value::Null,
            is_written: false,
            template: tmpl.clone()
        }
    }

    pub fn write(&mut self, new: Value, context: &mut String, current_rule: Stachemu) {
        let out = reshape_interpolation(new, &self.template);

        if let Value::Object(map) = out {
            for (key, value) in map {
                merge_into(&mut self.buffer, &key, &value);

                if let Value::String(eaten) = value.clone() {
                    context.drain(..eaten.len());
                }
            }

            if let Stachemu::Default(eaten) = current_rule {
                context.drain(..eaten.len());
            }

        } else if out == Value::Null {
            self.buffer = out;
        }

        self.is_written = true;
    }   
}


fn reshape_interpolation(to_reshape: Value, template: &Model) -> Value {
    let mut new_map = Map::new();
    let next_index = template.now() + 1;

    if let Value::Object(map) = to_reshape.clone() {
        if map.len() != 1 {
            return to_reshape;
        }

        if let Some(Stachemu::Default(next)) = template.get(next_index) {
            for (key, value) in map {
                if let Value::String(mut content) = value.clone() {
                    for next_char in next.chars() {
                        if let Some(last_match) = content.pop() {
                            if last_match != next_char {
                                content.push(last_match);
                                break;
                            }
                        }
                    }

                    new_map.insert(key.clone(), Value::String(content));
                }
            }
        }
    }

    if new_map.is_empty() {
        to_reshape
    } else {
        Value::Object(new_map)
    }
}

fn merge_into(into: &Value, key: &String, value: &Value) -> Value {
    use std::slice::SliceConcatExt;

    match *into {
        Value::Object(ref map) => {
            if key.contains(".") {
                let mut keys: Vec<&str> = key.split(".").collect();
                if let Some(first) = keys.clone().get(0) {
                    let mut is_merged = false;

                    for (old_key, mut old_value) in map {
                        if old_key == first {
                            keys.remove(0);
                            merge_into(old_value, &keys.join("."), value);
                            is_merged = true;
                            break;
                        }
                    }
                }
            } else {
                //map.insert(key.clone(), value.clone());
            }

            unimplemented!()
        },
        Value::Null => {
            if key.contains(".") {
                let mut keys: Vec<&str> = key.split(".").collect();
                let mut new_json = Value::Object(Map::new());

                for key in keys {
                    new_json = merge_into(&new_json, &String::from(key), value);
                }
            } else {

            }

            unimplemented!()
        },
        _ => unimplemented!()
    }
}