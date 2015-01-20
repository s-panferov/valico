#![allow(unstable)]

extern crate "rustc-serialize" as serialize;
extern crate regex;

use serialize::json;

pub use param::Param;
pub use builder::Builder;
pub use coercers::{
    Coercer,
    StringCoercer,
    I64Coercer,
    U64Coercer,
    F64Coercer,
    BooleanCoercer,
    NullCoercer,
    ListCoercer,
    ObjectCoercer,
};

pub use mutable_json::MutableJson;

pub type ValicoResult<T> = Result<T, json::Object>;

mod builder;
mod coercers;
mod helpers;
mod param;
mod validation;
mod mutable_json;

pub fn i64() -> Box<coercers::Coercer + Send + Sync> { Box::new(coercers::I64Coercer) }
pub fn u64() -> Box<coercers::Coercer + Send + Sync> { Box::new(coercers::U64Coercer) }
pub fn f64() -> Box<coercers::Coercer + Send + Sync> { Box::new(coercers::F64Coercer) }
pub fn string() -> Box<coercers::Coercer + Send + Sync> { Box::new(coercers::StringCoercer) }
pub fn boolean() -> Box<coercers::Coercer + Send + Sync> { Box::new(coercers::BooleanCoercer) }
pub fn null() -> Box<coercers::Coercer + Send + Sync> { Box::new(coercers::NullCoercer) }
pub fn list() -> Box<coercers::Coercer + Send + Sync> { Box::new(coercers::ListCoercer::new()) }
pub fn list_of(coercer: Box<coercers::Coercer + Send + Sync>) -> Box<coercers::Coercer + Send + Sync> { Box::new(coercers::ListCoercer::of_type(coercer)) }
pub fn object() -> Box<coercers::Coercer + Send + Sync> { Box::new(coercers::ObjectCoercer) }

