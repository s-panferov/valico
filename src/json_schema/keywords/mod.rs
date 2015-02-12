use rustc_serialize::json;
use std::fmt;
use std::rc;
use std::collections;

use super::schema;
use super::validators;

pub type KeywordResult = Result<Option<validators::BoxedValidator>, schema::SchemaError>;
pub type KeywordPair = (Vec<&'static str>, Box<Keyword + 'static>);
pub type KeywordPairs = Vec<KeywordPair>;
pub type KeywordMap = collections::HashMap<&'static str, rc::Rc<KeywordConsumer>>;

pub trait Keyword: Sync {
    fn compile(&self, &json::Json, &schema::WalkContext) -> KeywordResult;
}

impl fmt::Debug for Keyword + 'static {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("<keyword>")
    }
}

macro_rules! keyword_key_exists {
    ($val:expr, $key:expr) => {{
        let maybe_val = $val.find($key);
        if maybe_val.is_none() {
            return Ok(None)
        } else {
           maybe_val.unwrap() 
        }
    }}
}

pub mod multiple_of;
pub mod maxmin;
#[macro_use]
pub mod maxmin_length;
pub mod maxmin_items;
pub mod pattern;
pub mod unique_items;
pub mod items;
pub mod maxmin_properties;
pub mod required;
pub mod properties;
pub mod dependencies;
pub mod enum_;
pub mod type_;
pub mod of;
pub mod ref_;
pub mod not;

pub fn default() -> KeywordMap {
    let mut default: KeywordPairs = vec![];
    default.push((vec!["multipleOf"], Box::new(multiple_of::MultipleOf)));
    default.push((vec!["maximum", "exclusiveMaximum"], Box::new(maxmin::Maximum)));
    default.push((vec!["minimum", "exclusiveMinimum"], Box::new(maxmin::Minimum)));
    default.push((vec!["maxLength"], Box::new(maxmin_length::MaxLength)));
    default.push((vec!["minLength"], Box::new(maxmin_length::MinLength)));
    default.push((vec!["pattern"], Box::new(pattern::Pattern)));
    default.push((vec!["maxItems"], Box::new(maxmin_items::MaxItems)));
    default.push((vec!["minItems"], Box::new(maxmin_items::MinItems)));
    default.push((vec!["uniqueItems"], Box::new(unique_items::UniqueItems)));
    default.push((vec!["items", "additionalItems"], Box::new(items::Items)));
    default.push((vec!["maxProperties"], Box::new(maxmin_properties::MaxProperties)));
    default.push((vec!["minProperties"], Box::new(maxmin_properties::MinProperties)));
    default.push((vec!["required"], Box::new(required::Required)));
    default.push((vec!["properties", "additionalProperties", "patternProperties"], Box::new(properties::Properties)));
    default.push((vec!["dependencies"], Box::new(dependencies::Dependencies)));
    default.push((vec!["enum"], Box::new(enum_::Enum)));
    default.push((vec!["type"], Box::new(type_::Type)));
    default.push((vec!["allOf"], Box::new(of::AllOf)));
    default.push((vec!["anyOf"], Box::new(of::AnyOf)));
    default.push((vec!["oneOf"], Box::new(of::OneOf)));
    default.push((vec!["$ref"], Box::new(ref_::Ref)));
    default.push((vec!["not"], Box::new(not::Not)));

    let mut map = collections::HashMap::new();
    decouple_keywords(default, &mut map);

    map
}

#[derive(Debug)]
pub struct KeywordConsumer {
    pub keys: Vec<&'static str>,
    pub keyword: Box<Keyword + 'static>
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

pub fn decouple_keywords(keyword_pairs: KeywordPairs, 
                         map: &mut KeywordMap) {
    for keyword_pair in keyword_pairs.into_iter() {
        decouple_keyword(keyword_pair, map)
    }
}

pub fn decouple_keyword(keyword_pair: KeywordPair, 
                        map: &mut KeywordMap) {
    let (keys, keyword) = keyword_pair;
    let consumer = rc::Rc::new(KeywordConsumer { keys: keys.clone(), keyword: keyword });
    for key in keys.iter() {
        map.insert(key, consumer.clone());
    }
}
