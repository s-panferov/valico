use rustc_serialize::json::{self};

pub fn has_value(obj: &json::Object, key: &String) -> bool {
    match obj.get(key) {
        Some(_) => true,
        None => false
    }
}
