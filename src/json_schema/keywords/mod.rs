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
pub mod maximum;
pub mod minimum;
pub mod max_length;

pub fn default() -> Keywords {
    let mut default = vec![];
    default.push(Box::new(multiple_of::MultipleOf) as Box<Keyword>);
    default.push(Box::new(maximum::Maximum) as Box<Keyword>);
    default.push(Box::new(minimum::Minimum) as Box<Keyword>);
    default.push(Box::new(max_length::MaxLength) as Box<Keyword>);
    default
}
