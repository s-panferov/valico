
extern crate serialize;

use serialize::json;
use serialize::json::{Json, JsonObject, ToJson, I64};
use std::collections::TreeMap;

use mutable_json::MutableJson;

mod mutable_json;

#[deriving(Show)]
pub enum ValicoErrorKind {
	PresenceError,
	CoerceError,
	ValidationError,
    Other
}

#[deriving(Show)]
pub struct ValicoError {
    pub kind: ValicoErrorKind,
    pub message: String
}

pub type ValicoResult<T> = Result<T, TreeMap<String, ValicoError>>;

trait Coercer: Send + Sync {
	fn coerce(&self, &Json, &Option<Builder>) -> ValicoResult<Option<Json>>;
}

fn single_error(err: ValicoError) -> TreeMap<String, ValicoError> {
	let mut tree = TreeMap::new();
	tree.insert("0".to_string(), err);

	tree
}

fn singe_coerce_error(err_msg: String) -> TreeMap<String, ValicoError> {
	let mut tree = TreeMap::new();
	let error = ValicoError {
		kind: CoerceError,
		message: err_msg
	};
	tree.insert("0".to_string(), error);

	tree
}


struct StringCoercer;

impl Coercer for StringCoercer {
	fn coerce(&self, val: &Json, _: &Option<Builder>) -> ValicoResult<Option<Json>> {
		if val.is_string() {
			Ok(None)
		} else if val.is_number() {
			Ok(Some(val.to_string().to_json()))
		} else {
			Err(
				singe_coerce_error(format!("Can't coerce object {} to string", val))
			)
		}
	}
}

struct I64Coercer;

impl Coercer for I64Coercer {
	fn coerce(&self, val: &Json, _: &Option<Builder>) -> ValicoResult<Option<Json>> {
		if val.is_i64() {
			return Ok(None)
		} else if val.is_u64() {
			let val = val.as_u64().unwrap();
			return Ok(Some((val as i64).to_json()));
		} else if val.is_f64() {
			let val = val.as_f64().unwrap();
			return Ok(Some((val as i64).to_json()));
		} else if val.is_string() {
			let val = val.as_string().unwrap();
			let converted: Option<i64> = from_str(val);
			match converted {
				Some(num) => Ok(Some(num.to_json())),
				None => Err(singe_coerce_error(format!("Can't string value {} to i64", val)))
			}
		} else {
			Err(singe_coerce_error(format!("Can't coerce object {} to i64", val)))
		}
	}
}

struct U64Coercer;

impl Coercer for U64Coercer {
	fn coerce(&self, val: &Json, _: &Option<Builder>) -> ValicoResult<Option<Json>> {
		if val.is_u64() {
			return Ok(None)
		} else if val.is_i64() {
			let val = val.as_i64().unwrap();
			return Ok(Some((val as u64).to_json()));
		} else if val.is_f64() {
			let val = val.as_f64().unwrap();
			return Ok(Some((val as u64).to_json()));
		} else if val.is_string() {
			let val = val.as_string().unwrap();
			let converted: Option<u64> = from_str(val);
			match converted {
				Some(num) => Ok(Some(num.to_json())),
				None => Err(singe_coerce_error(format!("Can't string value {} to u64", val)))
			}
		} else {
			Err(singe_coerce_error(format!("Can't coerce object {} to u64", val)))
		}
	}
}

struct F64Coercer;

impl Coercer for F64Coercer {
	fn coerce(&self, val: &Json, _: &Option<Builder>) -> ValicoResult<Option<Json>> {
		if val.is_f64() {
			return Ok(None)
		} else if val.is_i64() {
			let val = val.as_i64().unwrap();
			return Ok(Some((val as f64).to_json()));
		} else if val.is_u64() {
			let val = val.as_u64().unwrap();
			return Ok(Some((val as f64).to_json()));
		} else if val.is_string() {
			let val = val.as_string().unwrap();
			let converted: Option<f64> = from_str(val);
			match converted {
				Some(num) => Ok(Some(num.to_json())),
				None => Err(singe_coerce_error(format!("Can't coerce string value {} to f64", val)))
			}
		} else {
			Err(singe_coerce_error(format!("Can't coerce object {} to f64", val)))
		}
	}
}

struct BooleanCoercer;

impl Coercer for BooleanCoercer {
	fn coerce(&self, val: &Json, _: &Option<Builder>) -> ValicoResult<Option<Json>> {
		if val.is_boolean() {
			Ok(None)
		} else if val.is_string() {
			let val = val.as_string().unwrap();
			if val == "true" {
				Ok(Some(true.to_json()))
			} else if val == "false" {
				Ok(Some(false.to_json()))
			} else {
				Err(singe_coerce_error(format!("Can't coerce string value {} to boolean. Correct values is 'true' and 'false'", val)))
			}
		} else {
			Err(singe_coerce_error(format!("Can't coerce object {} to boolean", val)))
		}
	}
}

struct NullCoercer;

impl Coercer for NullCoercer {
	fn coerce(&self, val: &Json, _: &Option<Builder>) -> ValicoResult<Option<Json>> {
		if val.is_null() {
			Ok(None)
		} else if val.is_string() {
			let val = val.as_string().unwrap();
			if val == "" {
				Ok(Some(json::Null))
			} else {
				Err(singe_coerce_error(format!("Can't coerce string value {} to null. Correct value is only empty string", val)))
			}
		} else {
			Err(singe_coerce_error(format!("Can't object {} to null", val)))
		}
	}
}

struct ListCoercer {
	sub_coercer: Option<Box<Coercer>>
}

impl ListCoercer {
	pub fn new() -> ListCoercer {
		ListCoercer {
			sub_coercer: None
		}
	}

	pub fn with_type(sub_coercer: Box<Coercer>) -> ListCoercer {
		ListCoercer {
			sub_coercer: Some(sub_coercer)
		}
	}
}

// impl Coercer for ListCoercer {
// 	fn coerce(&self, val: &Json, _: &Option<Builder>) -> ValicoResult<Option<Json>> {
// 		if val.is_list() {

// 		} else {
// 			Err(singe_coerce_error(format!("Can't object {} to null", val)))
// 		}
// 	}
// }

struct Param {
	pub name: String,
	pub coercer: Box<Coercer>,
	pub extra: Option<Builder>
}

impl Param {
	pub fn new(name: &str, coercer: Box<Coercer>) -> Param {
		Param {
			name: name.to_string(),
			coercer: coercer,
			extra: None
		}
	}

	pub fn new_with_extra(name: &str, coercer: Box<Coercer>, extra: Builder) -> Param {
		Param {
			name: name.to_string(),
			coercer: coercer,
			extra: Some(extra)
		}
	}

	pub fn process(&self, val: &mut Json) -> ValicoResult<Option<Json>> {
		return (*self.coercer).coerce(val, &self.extra)
	}
}

struct Builder {
	requires: Vec<Param>,
	optional: Vec<Param>
}

fn has_value(obj: &JsonObject, key: &String) -> bool {
	match obj.find(key) {
		Some(_) => true,
		None => false
	}
}

impl Builder {

	pub fn new() -> Builder {
		Builder {
			requires: vec![],
			optional: vec![]
		}
	}

	fn from_function(rules: |params: &mut Builder|) -> Builder {
		let mut builder = Builder::new();
		rules(&mut builder);

		builder
	}

	fn req(&mut self, name: &str) {
		let params = Param::new(name, box StringCoercer);
		self.requires.push(params);
	}

	fn req_type(&mut self, name: &str, coercer: Box<Coercer>) {
		let params = Param::new(name, coercer);
		self.requires.push(params);
	}

	fn req_nest(&mut self, name: &str, coercer: Box<Coercer>, extra: |params: &mut Builder|) {
		let extra_builder = Builder::from_function(extra);
		let params = Param::new_with_extra(name, coercer, extra_builder);
		self.requires.push(params);
	}

	fn process(&self, tree: &mut JsonObject)  {
		
		for param in self.requires.iter() {
			let ref name = param.name;
			let present = has_value(tree, name);
			if present {
				match param.process(tree.find_mut(name).unwrap()) {
					Ok(result) => { 
						match result {
							Some(new_value) => { tree.insert(name.clone(), new_value); },
							None => ()
						}
					},
					Err(_) => ()
				}
			} else {
				// required error!
			}

		}

	}

	// pub fn optional(name: &str, kind: Coeletrcer);
	// pub fn group(name: &str);
	// pub fn mutually_exclusive();
}

#[test]
fn it_works() {
	use serialize::json;

	let params = Builder::from_function(|params: &mut Builder| {
		params.req("name");
		params.req_type("test", box StringCoercer);
		params.req_nest("test", box StringCoercer, |params: &mut Builder| {
			params.req("type");
		});
	});

	let mut json = json::from_str(r#"{"name": 1}"#).unwrap();
	params.process(json.as_object_mut().unwrap());
}
