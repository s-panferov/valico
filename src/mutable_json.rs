use serialize::json;

pub trait MutableJson {
    fn as_object_mut<'a>(&'a mut self) -> Option<&'a mut json::Object>;
    fn as_array_mut<'a>(&'a mut self) -> Option<&'a mut json::Array>;
}

impl MutableJson for json::Json {
    
    /// If the Json value is an Object, returns the associated BTreeMap.
    /// Returns None otherwise.
    fn as_object_mut<'a>(&'a mut self) -> Option<&'a mut json::Object> {
        match self {
            &mut json::Json::Object(ref mut map) => Some(&mut*map),
            _ => None
        }
    }

    fn as_array_mut<'a>(&'a mut self) -> Option<&'a mut json::Array> {
        match self {
            &mut json::Json::Array(ref mut list) => Some(&mut *list),
            _ => None
        }
    }

}