
use serialize::json;

use builder::Builder;
use coercers::Coercer;
use ValicoResult;

pub struct Param {
	pub name: String,
	pub coercer: Option<Box<Coercer>>,
	pub extra: Option<Builder>
}

impl Param {
	pub fn new(name: &str) -> Param {
		Param {
			name: name.to_string(),
			coercer: None,
			extra: None
		}
	}

	pub fn new_with_coercer(name: &str, coercer: Box<Coercer>) -> Param {
		Param {
			name: name.to_string(),
			coercer: Some(coercer),
			extra: None
		}
	}

	pub fn new_with_extra(name: &str, coercer: Box<Coercer>, extra: Builder) -> Param {
		Param {
			name: name.to_string(),
			coercer: Some(coercer),
			extra: Some(extra)
		}
	}

	pub fn process(&self, val: &mut json::Json) -> ValicoResult<Option<json::Json>> {
		match self.coercer.as_ref() {
			Some(coercer) => coercer.coerce(val, self.extra.as_ref()),
			None => Ok(None)
		}
	}
}