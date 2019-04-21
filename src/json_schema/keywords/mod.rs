use serde_json::{Value};
use std::fmt;
use std::sync::Arc;
use std::collections;
use std::any;

use super::schema;
use super::validators;

pub type KeywordResult = Result<Option<validators::BoxedValidator>, schema::SchemaError>;
pub type KeywordPair = (Vec<&'static str>, Arc<Keyword + 'static>);
pub type KeywordPairs = Vec<KeywordPair>;
pub type KeywordMap = collections::HashMap<&'static str, Arc<KeywordConsumer>>;

pub trait Keyword: Send + Sync + any::Any {
    fn compile(&self, &Value, &schema::WalkContext) -> KeywordResult;

    fn is_exclusive(&self) -> bool {
        false
    }
}

impl<T: 'static + Send + Sync + any::Any> Keyword for T where T: Fn(&Value, &schema::WalkContext) -> KeywordResult {
    fn compile(&self, def: &Value, ctx: &schema::WalkContext) -> KeywordResult {
        self(def, ctx)
    }
}

impl fmt::Debug for Keyword + 'static {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("<keyword>")
    }
}

macro_rules! keyword_key_exists {
    ($val:expr, $key:expr) => {{
        let maybe_val = $val.get($key);
        if maybe_val.is_none() {
            return Ok(None)
        } else {
           maybe_val.unwrap()
        }
    }}
}

#[macro_use]
pub mod maxmin_length;

pub mod const_;
pub mod contains;
pub mod dependencies;
pub mod enum_;
pub mod format;
pub mod items;
pub mod maxmin;
pub mod maxmin_items;
pub mod maxmin_properties;
pub mod multiple_of;
pub mod not;
pub mod of;
pub mod pattern;
pub mod properties;
pub mod property_names;
pub mod ref_;
pub mod required;
pub mod type_;
pub mod unique_items;

pub fn default() -> KeywordMap {
    let mut map = collections::HashMap::new();

    decouple_keyword((vec!["$ref"], Arc::new(ref_::Ref)), &mut map);
    decouple_keyword((vec!["allOf"], Arc::new(of::AllOf)), &mut map);
    decouple_keyword((vec!["anyOf"], Arc::new(of::AnyOf)), &mut map);
    decouple_keyword((vec!["const"], Arc::new(const_::Const)), &mut map);
    decouple_keyword((vec!["contains"], Arc::new(contains::Contains)), &mut map);
    decouple_keyword((vec!["dependencies"], Arc::new(dependencies::Dependencies)), &mut map);
    decouple_keyword((vec!["enum"], Arc::new(enum_::Enum)), &mut map);
    decouple_keyword((vec!["exclusiveMaximum"], Arc::new(maxmin::ExclusiveMaximum)), &mut map);
    decouple_keyword((vec!["exclusiveMinimum"], Arc::new(maxmin::ExclusiveMinimum)), &mut map);
    decouple_keyword((vec!["items", "additionalItems"], Arc::new(items::Items)), &mut map);
    decouple_keyword((vec!["maxItems"], Arc::new(maxmin_items::MaxItems)), &mut map);
    decouple_keyword((vec!["maxLength"], Arc::new(maxmin_length::MaxLength)), &mut map);
    decouple_keyword((vec!["maxProperties"], Arc::new(maxmin_properties::MaxProperties)), &mut map);
    decouple_keyword((vec!["maximum"], Arc::new(maxmin::Maximum)), &mut map);
    decouple_keyword((vec!["minItems"], Arc::new(maxmin_items::MinItems)), &mut map);
    decouple_keyword((vec!["minLength"], Arc::new(maxmin_length::MinLength)), &mut map);
    decouple_keyword((vec!["minProperties"], Arc::new(maxmin_properties::MinProperties)), &mut map);
    decouple_keyword((vec!["minimum"], Arc::new(maxmin::Minimum)), &mut map);
    decouple_keyword((vec!["multipleOf"], Arc::new(multiple_of::MultipleOf)), &mut map);
    decouple_keyword((vec!["not"], Arc::new(not::Not)), &mut map);
    decouple_keyword((vec!["oneOf"], Arc::new(of::OneOf)), &mut map);
    decouple_keyword((vec!["pattern"], Arc::new(pattern::Pattern)), &mut map);
    decouple_keyword((vec!["properties", "additionalProperties", "patternProperties"], Arc::new(properties::Properties)), &mut map);
    decouple_keyword((vec!["propertyNames"], Arc::new(property_names::PropertyNames)), &mut map);
    decouple_keyword((vec!["required"], Arc::new(required::Required)), &mut map);
    decouple_keyword((vec!["type"], Arc::new(type_::Type)), &mut map);
    decouple_keyword((vec!["uniqueItems"], Arc::new(unique_items::UniqueItems)), &mut map);

    map
}

#[derive(Debug)]
pub struct KeywordConsumer {
    pub keys: Vec<&'static str>,
    pub keyword: Arc<Keyword + 'static>
}

impl KeywordConsumer {
    pub fn consume(&self, set: &mut collections::HashSet<&str>) {
        for key in self.keys.iter() {
            if set.contains(key) {
                set.remove(key);
            }
        }
    }
}

pub fn decouple_keyword(keyword_pair: KeywordPair,
                        map: &mut KeywordMap) {
    let (keys, keyword) = keyword_pair;
    let consumer = Arc::new(KeywordConsumer { keys: keys.clone(), keyword: keyword });
    for key in keys.iter() {
        map.insert(key, consumer.clone());
    }
}
