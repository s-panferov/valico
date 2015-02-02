mod builder;
mod coercers;
mod helpers;
mod param;
pub mod errors;
#[macro_use] pub mod validators;

pub use self::param::Param;
pub use self::builder::Builder;
pub use self::coercers::{
    PrimitiveType,
    Coercer,
    StringCoercer,
    I64Coercer,
    U64Coercer,
    F64Coercer,
    BooleanCoercer,
    NullCoercer,
    ArrayCoercer,
    ObjectCoercer,
};

pub fn i64() -> Box<coercers::Coercer + Send + Sync> { Box::new(coercers::I64Coercer) }
pub fn u64() -> Box<coercers::Coercer + Send + Sync> { Box::new(coercers::U64Coercer) }
pub fn f64() -> Box<coercers::Coercer + Send + Sync> { Box::new(coercers::F64Coercer) }
pub fn string() -> Box<coercers::Coercer + Send + Sync> { Box::new(coercers::StringCoercer) }
pub fn boolean() -> Box<coercers::Coercer + Send + Sync> { Box::new(coercers::BooleanCoercer) }
pub fn null() -> Box<coercers::Coercer + Send + Sync> { Box::new(coercers::NullCoercer) }
pub fn array() -> Box<coercers::Coercer + Send + Sync> { Box::new(coercers::ArrayCoercer::new()) }
pub fn array_of(coercer: Box<coercers::Coercer + Send + Sync>) -> Box<coercers::Coercer + Send + Sync> { Box::new(coercers::ArrayCoercer::of_type(coercer)) }
pub fn object() -> Box<coercers::Coercer + Send + Sync> { Box::new(coercers::ObjectCoercer) }

pub type DslResult<T> = Result<T, super::common::error::ValicoErrors>;