use serialize::json;
use serialize::json::{Json, JsonObject, JsonArray};

pub trait MutableJson {
    fn as_object_mut<'a>(&'a mut self) -> Option<&'a mut JsonObject>;
    fn as_array_mut<'a>(&'a mut self) -> Option<&'a mut JsonArray>;
}

impl MutableJson for Json {
    
    /// If the Json value is an Object, returns the associated TreeMap.
    /// Returns None otherwise.
    fn as_object_mut<'a>(&'a mut self) -> Option<&'a mut JsonObject> {
        match self {
            &json::Object(ref mut map) => Some(&mut*map),
            _ => None
        }
    }

    fn as_array_mut<'a>(&'a mut self) -> Option<&'a mut JsonArray> {
        match self {
            &json::Array(ref mut list) => Some(&mut *list),
            _ => None
        }
    }

}