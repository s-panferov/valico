use std::collections::TreeMap;
use serialize::json::{Object, ToJson};

pub fn validation_error(err: String) -> Object {
    let mut tree = TreeMap::new();
    tree.insert("validation".to_string(), [err].to_json());
    tree
}

pub fn coerce_error(err: String) -> Object {
    let mut tree = TreeMap::new();
    tree.insert("coercion".to_string(), [err].to_json());
    tree
}

pub fn has_value(obj: &Object, key: &String) -> bool {
    match obj.get(key) {
        Some(_) => true,
        None => false
    }
}
