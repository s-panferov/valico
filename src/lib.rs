
extern crate serialize;

use serialize::json;
use serialize::json::{Json, JsonObject, ToJson, I64};
use std::collections::TreeMap;

use mutable_json::MutableJson;

mod mutable_json;

pub type ValicoResult<T> = Result<T, JsonObject>;

fn single_coerce_error(err: String) -> JsonObject {
	let mut tree = TreeMap::new();
	tree.insert("type".to_string(), "coerce".to_string().to_json());
	tree.insert("message".to_string(), err.to_json());

	tree
}

trait Coercer: Send + Sync {
	fn coerce(&self, &mut Json, Option<&Builder>) -> ValicoResult<Option<Json>>;
}

struct StringCoercer;

impl Coercer for StringCoercer {
	fn coerce(&self, val: &mut Json, _: Option<&Builder>) -> ValicoResult<Option<Json>> {
		if val.is_string() {
			Ok(None)
		} else if val.is_number() {
			Ok(Some(val.to_string().to_json()))
		} else {
			Err(
				single_coerce_error(format!("Can't coerce object {} to string", val))
			)
		}
	}
}

struct I64Coercer;

impl Coercer for I64Coercer {
	fn coerce(&self, val: &mut Json, _: Option<&Builder>) -> ValicoResult<Option<Json>> {
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
				None => Err(single_coerce_error(format!("Can't coerce string value {} to i64", val)))
			}
		} else {
			Err(single_coerce_error(format!("Can't coerce object {} to i64", val)))
		}
	}
}

struct U64Coercer;

impl Coercer for U64Coercer {
	fn coerce(&self, val: &mut Json, _: Option<&Builder>) -> ValicoResult<Option<Json>> {
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
				None => Err(single_coerce_error(format!("Can't coerce string value {} to u64", val)))
			}
		} else {
			Err(single_coerce_error(format!("Can't coerce object {} to u64", val)))
		}
	}
}

struct F64Coercer;

impl Coercer for F64Coercer {
	fn coerce(&self, val: &mut Json, _: Option<&Builder>) -> ValicoResult<Option<Json>> {
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
				None => Err(single_coerce_error(format!("Can't coerce string value {} to f64", val)))
			}
		} else {
			Err(single_coerce_error(format!("Can't coerce object {} to f64", val)))
		}
	}
}

struct BooleanCoercer;

impl Coercer for BooleanCoercer {
	fn coerce(&self, val: &mut Json, _: Option<&Builder>) -> ValicoResult<Option<Json>> {
		if val.is_boolean() {
			Ok(None)
		} else if val.is_string() {
			let val = val.as_string().unwrap();
			if val == "true" {
				Ok(Some(true.to_json()))
			} else if val == "false" {
				Ok(Some(false.to_json()))
			} else {
				Err(single_coerce_error(format!("Can't coerce string value {} to boolean. Correct values is 'true' and 'false'", val)))
			}
		} else {
			Err(single_coerce_error(format!("Can't coerce object {} to boolean", val)))
		}
	}
}

struct NullCoercer;

impl Coercer for NullCoercer {
	fn coerce(&self, val: &mut Json, _: Option<&Builder>) -> ValicoResult<Option<Json>> {
		if val.is_null() {
			Ok(None)
		} else if val.is_string() {
			let val = val.as_string().unwrap();
			if val == "" {
				Ok(Some(json::Null))
			} else {
				Err(single_coerce_error(format!("Can't coerce string value {} to null. Correct value is only empty string", val)))
			}
		} else {
			Err(single_coerce_error(format!("Can't coerce object {} to null", val)))
		}
	}
}

struct ListCoercer {
	sub_coercer: Option<Box<Coercer + Send + Sync>>
}

impl ListCoercer {
	pub fn new() -> ListCoercer {
		ListCoercer {
			sub_coercer: None
		}
	}

	pub fn with_type(sub_coercer: Box<Coercer + Send + Sync>) -> ListCoercer {
		ListCoercer {
			sub_coercer: Some(sub_coercer)
		}
	}
}

impl Coercer for ListCoercer {
	fn coerce(&self, val: &mut Json, extra: Option<&Builder>) -> ValicoResult<Option<Json>> {
		if val.is_list() {
			let list = val.as_list_mut().unwrap();
			let mut errors = TreeMap::new();
			if extra.is_some() {
				for (idx, item) in list.iter_mut().enumerate() {
					if item.is_object() {
						// todo match
						match extra.unwrap().process(item.as_object_mut().unwrap()) {
							Ok(()) => (),
							Err(err) => { errors.insert(idx.to_string(), err.to_json()); }
						}
					} else {
						errors.insert(idx.to_string(), format!("List item {} is not and object", item).to_json());
					}
				}

				if errors.len() == 0 {
					Ok(None)
				} else {
					Err(errors)
				}
			} else if self.sub_coercer.is_some() {
				let sub_coercer = self.sub_coercer.as_ref().unwrap();
				let mut errors = TreeMap::new();
				for i in range(0, list.len() - 1) {
					match sub_coercer.coerce(list.get_mut(i), None) {
						Ok(Some(value)) => {
							list.remove(i);
							list.insert(i, value);
						},
						Ok(None) => (),
						Err(err) => {
							errors.insert(i.to_string(), err.to_json());
						}
					}
				}

				if errors.len() == 0 {
					Ok(None)
				} else {
					Err(errors)
				}
			} else {
				Ok(None)
			}
		} else {
			Err(single_coerce_error(format!("Can't coerce object {} to null", val)))
		}
	}
}

struct ObjectCoercer;

impl Coercer for ObjectCoercer {
	fn coerce(&self, val: &mut Json, extra: Option<&Builder>) -> ValicoResult<Option<Json>> {
		if val.is_object() {
			// todo match
			extra.unwrap().process(val.as_object_mut().unwrap());
			Ok(None)
		} else {
			Err(single_coerce_error(format!("Can't coerce non-object value {} to object", val)))
		}
	}
}

struct Param {
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

	pub fn process(&self, val: &mut Json) -> ValicoResult<Option<Json>> {
		match self.coercer.as_ref() {
			Some(coercer) => coercer.coerce(val, self.extra.as_ref()),
			None => Ok(None)
		}
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
		let params = Param::new(name);
		self.requires.push(params);
	}

	fn req_type(&mut self, name: &str, coercer: Box<Coercer>) {
		let params = Param::new_with_coercer(name, coercer);
		self.requires.push(params);
	}

	fn req_nest(&mut self, name: &str, coercer: Box<Coercer>, extra: |params: &mut Builder|) {
		let extra_builder = Builder::from_function(extra);
		let params = Param::new_with_extra(name, coercer, extra_builder);
		self.requires.push(params);
	}

	fn process(&self, tree: &mut JsonObject) -> ValicoResult<()>  {
		
		let mut errors = TreeMap::new();

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
					Err(err) => {
						errors.insert(name.to_string(), err.to_json());
					}
				}
			} else {
				errors.insert(name.to_string(), "Field is required but missing".to_string().to_json());
			}
		}

		if errors.len() == 0 {
			Ok(())
		} else {
			Err(errors)
		}
	}

	// pub fn optional(name: &str, kind: Coeletrcer);
	// pub fn group(name: &str);
	// pub fn mutually_exclusive();
}

fn test_result(params: &Builder, body: &str, result: &str) {
	let mut obj = json::from_str(body);
	match obj {
		Ok(mut json) => { 
			match params.process(json.as_object_mut().unwrap()) {
				Ok(()) => {
					return assert_eq!(json.to_string(), result.to_string());
				},
				Err(err) => {
					fail!("Error during process: {}", err.to_json().to_pretty_str());
				}
			}
		},
		Err(_) => {
			fail!("Invalid JSON");
		}
	}
}

fn test_get_error(params: &Builder, body: &str) -> JsonObject {
	let mut obj = json::from_str(body);
	match obj {
		Ok(mut json) => { 
			match params.process(json.as_object_mut().unwrap()) {
				Ok(()) => {
					fail!("Success responce when we await some errors");
				},
				Err(err) => {
					return err;
				}
			}
		},
		Err(_) => {
			fail!("Invalid JSON");
		}
	}
}

#[test]
fn is_process_simple_require() {

	let params = Builder::from_function(|params: &mut Builder| {
		params.req("name");
	});

	test_result(&params, r#"{"name":1}"#, r#"{"name":1}"#);
	println!("{}", test_get_error(&params, r#"{}"#));
	fail!("");
}
