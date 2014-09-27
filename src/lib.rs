
extern crate serialize;

use serialize::json;

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

pub type ValicoResult<T> = Result<T, json::JsonObject>;

mod builder;
mod coercers;
mod helpers;
mod param;
mod mutable_json;
