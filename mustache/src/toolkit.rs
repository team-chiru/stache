use serde_json::Value;

pub fn interpolate(key: &String, json: &Value) -> Option<String> {
    let mut data = Some(json);

    if *key != String::default() {
        let path = String::from("/") + &key.replace(".", "/");
        data = data.unwrap().pointer(&path);
    }

    if let Some(value) = data {
        use self::Value::*;

        match *value {
            Bool(b) => {
                Some(
                    if b { "true".to_string() } else { "false".to_string() }
                )
            },
            String(ref s) => Some(s.clone()),
            Number(ref n) => {
                if let Some(s) = n.as_i64() {
                    Some(s.to_string())
                } else if let Some(s) = n.as_u64() {
                    Some(s.to_string())
                } else if let Some(s) = n.as_f64() {
                    Some(s.to_string())
                } else {
                    None
                }
            },
            _ => None
        }
    } else {
        None
    }
}

// needs global each time
pub fn interpolate_section(key: &String, context: &Value) -> Option<Vec<Value>> {
    let path = String::from("/") + &key.replace(".", "/");

    if let Some(json) = context.pointer(&path) {
        use self::Value::*;

        match json.clone() {
            Bool(false) | Null => None,
            Array(values) => {
                if values.is_empty() {
                    None
                } else {
                    Some(values)
                }
            },
            _ => Some(vec![json.clone()])
        }
    } else {
        None
    }
}
