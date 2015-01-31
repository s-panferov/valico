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