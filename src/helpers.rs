use std::collections::TreeMap;
use serialize::json::{JsonObject, ToJson};

pub fn single_validation_error(err: String) -> JsonObject {
	let mut tree = TreeMap::new();
	tree.insert("validation".to_string(), [err].to_json());
	tree
}

pub fn single_coerce_error(err: String) -> JsonObject {
	let mut tree = TreeMap::new();
	tree.insert("coercion".to_string(), [err].to_json());
	tree
}

pub fn has_value(obj: &JsonObject, key: &String) -> bool {
	match obj.find(key) {
		Some(_) => true,
		None => false
	}
}
