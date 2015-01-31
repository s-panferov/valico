use serialize::json;
use std::fmt;

use super::schema;
use super::validators;

pub type KeywordResult = Result<Option<validators::BoxedValidator>, schema::SchemaError>;
pub type Keywords = Vec<Box<Keyword + 'static>>;

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

pub fn default() -> Keywords {
    let mut default = vec![];
    default.push(Box::new(multiple_of::MultipleOf) as Box<Keyword>);
    default.push(Box::new(maxmin::Maximum) as Box<Keyword>);
    default.push(Box::new(maxmin::Minimum) as Box<Keyword>);
    default.push(Box::new(maxmin_length::MaxLength) as Box<Keyword>);
    default.push(Box::new(maxmin_length::MinLength) as Box<Keyword>);
    default.push(Box::new(pattern::Pattern) as Box<Keyword>);
    default.push(Box::new(maxmin_items::MaxItems) as Box<Keyword>);
    default.push(Box::new(maxmin_items::MinItems) as Box<Keyword>);
    default.push(Box::new(unique_items::UniqueItems) as Box<Keyword>);
    default.push(Box::new(items::Items) as Box<Keyword>);
    default
}