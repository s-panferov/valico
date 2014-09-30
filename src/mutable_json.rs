use serialize::json;
use serialize::json::{Json, JsonObject, JsonList};

pub trait MutableJson {
    fn as_object_mut<'a>(&'a mut self) -> Option<&'a mut JsonObject>;
    fn as_list_mut<'a>(&'a mut self) -> Option<&'a mut JsonList>;
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

    fn as_list_mut<'a>(&'a mut self) -> Option<&'a mut JsonList> {
        match self {
            &json::List(ref mut list) => Some(&mut *list),
            _ => None
        }
    }

}