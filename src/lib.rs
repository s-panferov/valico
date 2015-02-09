#![feature(core)]
#![feature(collections)]
#![feature(plugin)]

extern crate "rustc-serialize" as rustc_serialize;
extern crate regex;
extern crate collections;
extern crate url;
extern crate jsonway;
#[macro_use] #[no_link]
extern crate mopa;

pub use mutable_json::MutableJson;

mod mutable_json;
#[macro_use] pub mod common;
pub mod json_dsl;
pub mod json_schema;

pub use common::error::{ValicoErrors};