use serde::Serialize;
use std::collections::HashMap;

use crate::errors::StringTemplaterError;

pub fn parse_to_hashmap<T: ?Sized + Serialize>(
    value: &T,
) -> Result<HashMap<String, String>, StringTemplaterError> {
    match serde_json::to_value(value) {
        Ok(serialized) => {
            let mut map = HashMap::new();
            flatten("", &serialized, &mut map);
            Ok(map)
        }
        Err(err) => Err(StringTemplaterError::SerializeError(err.to_string())),
    }
}

fn flatten(prefix: &str, value: &serde_json::Value, map: &mut HashMap<String, String>) {
    match value {
        serde_json::Value::Object(obj) => {
            for (k, v) in obj {
                let new_prefix = if prefix.is_empty() {
                    (*k).clone()
                } else {
                    format!("{}.{}", prefix, k)
                };
                flatten(&new_prefix, v, map);
            }
        }
        serde_json::Value::Array(arr) => {
            for (i, v) in arr.iter().enumerate() {
                let new_prefix = format!("{}.{}", prefix, i);
                flatten(&new_prefix, v, map);
            }
        }
        _ => {
            let _ = match value {
                serde_json::Value::String(s) => map.insert(prefix.to_string(), s.clone()),
                _ => map.insert(prefix.to_string(), value.to_string()),
            };
        }
    }
}
