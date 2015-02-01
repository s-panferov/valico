use std::str;
use std::fmt;

#[macro_use] pub mod helpers;
#[macro_use] pub mod keywords;
pub mod schema;
pub mod scope;
pub mod validators;
pub mod errors;

#[derive(Copy, Debug)]
pub enum PrimitiveType {
    Array,
    Boolean,
    Integer,
    Number,
    Null,
    Object,
    String,
}

impl str::FromStr for PrimitiveType {
    fn from_str(s: &str) -> Option<PrimitiveType> {
        match s {
            "array" => Some(PrimitiveType::Array),
            "boolean" => Some(PrimitiveType::Boolean),
            "integer" => Some(PrimitiveType::Integer),
            "number" => Some(PrimitiveType::Number),
            "null" => Some(PrimitiveType::Null),
            "object" => Some(PrimitiveType::Object),
            "string" => Some(PrimitiveType::String),
            _ => None
        }
    }
}

impl fmt::Display for PrimitiveType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            &PrimitiveType::Array => "array",
            &PrimitiveType::Boolean => "boolean",
            &PrimitiveType::Integer => "integer",
            &PrimitiveType::Number => "number",
            &PrimitiveType::Null => "null",
            &PrimitiveType::Object => "object",
            &PrimitiveType::String => "string",
        })
    }
}