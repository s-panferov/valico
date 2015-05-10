extern crate rustc_serialize;
extern crate regex;
extern crate url;
extern crate jsonway;
extern crate uuid;
extern crate phf;
#[macro_use] extern crate lazy_static;
extern crate typeable;
extern crate traitobject;

pub use mutable_json::MutableJson;

mod mutable_json;
#[macro_use] pub mod common;
pub mod json_dsl;
pub mod json_schema;

pub use common::error::{ValicoErrors};
