use url;

#[macro_use] pub mod helpers;
#[macro_use] pub mod keywords;
pub mod schema;
pub mod scope;
pub mod validators;
pub mod errors;

#[derive(Copy)]
pub enum PrimitiveType {
    Array,
    Boolean,
    Integer,
    Number,
    Null,
    Object,
    String,
}

#[derive(Debug)]
pub struct ValidationResult {
    valid: bool,
    errors: Vec<Box<super::common::error::ValicoError>>,
    missing: Vec<url::Url>
}