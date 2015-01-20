use std::collections;
use serialize::json::{self, ToJson};

pub fn validation_error(err: String) -> json::Object {
    let mut tree = collections::BTreeMap::new();
    tree.insert("validation".to_string(), [err].to_json());
    tree
}

pub fn coerce_error(err: String) -> json::Object {
    let mut tree = collections::BTreeMap::new();
    tree.insert("coercion".to_string(), [err].to_json());
    tree
}

pub fn has_value(obj: &json::Object, key: &String) -> bool {
    match obj.get(key) {
        Some(_) => true,
        None => false
    }
}
