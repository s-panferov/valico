
use serialize::json;
use serialize::json::{Json, ToJson};
use std::collections::TreeMap;

use mutable_json::MutableJson;
use builder::Builder;
use helpers::{single_coerce_error};
use ValicoResult;

pub trait Coercer: Send + Sync {
	fn coerce(&self, &mut Json, Option<&Builder>) -> ValicoResult<Option<Json>>;
}

pub struct StringCoercer;

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

pub struct I64Coercer;

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

pub struct U64Coercer;

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

pub struct F64Coercer;

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

pub struct BooleanCoercer;

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

pub struct NullCoercer;

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

pub struct ListCoercer {
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

pub struct ObjectCoercer;

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
