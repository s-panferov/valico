
use serialize::json;

use builder::Builder;
use coercers::Coercer;
use ValicoResult;

pub struct Param {
	pub name: String,
	pub coercer: Option<Box<Coercer>>,
	pub nest: Option<Builder>
}

impl Param {
	pub fn new(name: &str) -> Param {
		Param {
			name: name.to_string(),
			coercer: None,
			nest: None
		}
	}

	pub fn new_with_coercer(name: &str, coercer: Box<Coercer>) -> Param {
		Param {
			name: name.to_string(),
			coercer: Some(coercer),
			nest: None
		}
	}

	pub fn new_with_nest(name: &str, coercer: Box<Coercer>, nest: Builder) -> Param {
		Param {
			name: name.to_string(),
			coercer: Some(coercer),
			nest: Some(nest)
		}
	}

	pub fn process(&self, val: &mut json::Json) -> ValicoResult<Option<json::Json>> {
		match self.coercer.as_ref() {
			Some(coercer) => coercer.coerce(val, self.nest.as_ref()),
			None => Ok(None)
		}
	}
}