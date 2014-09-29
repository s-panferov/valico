
use serialize::json;
use serialize::json::{Json, ToJson};
use std::collections::TreeMap;
use regex::Regex;

use mutable_json::MutableJson;
use builder::Builder;
use coercers::Coercer;
use validation::{
	SingleParamValidator,
	AllowedValuesValidator,
	RejectedValuesValidator,
	FunctionValidator,
	RegexValidator
};
use ValicoResult;

use helpers::{validation_error};

pub struct Param {
	pub name: String,
	pub coercer: Option<Box<Coercer>>,
	pub nest: Option<Builder>,
	pub description: Option<String>,
	pub allow_null: bool,
	pub validators: Vec<Box<SingleParamValidator + Send + Sync>>,
	pub default: Option<Json>
}

impl Param {

	pub fn new(name: &str) -> Param {
		Param {
			name: name.to_string(),
			description: None,
			coercer: None,
			nest: None,
			allow_null: false,
			validators: vec![],
			default: None
		}
	}

	pub fn new_with_coercer(name: &str, coercer: Box<Coercer>) -> Param {
		Param {
			name: name.to_string(),
			description: None,
			coercer: Some(coercer),
			nest: None,
			allow_null: false,
			validators: vec![],
			default: None
		}
	}

	pub fn new_with_nest(name: &str, coercer: Box<Coercer>, nest: Builder) -> Param {
		Param {
			name: name.to_string(),
			description: None,
			coercer: Some(coercer),
			nest: Some(nest),
			allow_null: false,
			validators: vec![],
			default: None
		}
	}

	pub fn build(name: &str, build_def: |param: &mut Param|) -> Param {
		let mut param = Param::new(name);
		build_def(&mut param);

		param
	}

	pub fn desc(&mut self, description: &str) {
		self.description = Some(description.to_string());
	}

	pub fn coerce(&mut self, coercer: Box<Coercer>) {
		self.coercer = Some(coercer);
	}

	pub fn nest(&mut self, nest_def: |&mut Builder|) {
		self.nest = Some(Builder::build(nest_def));
	}

	pub fn allow_null(&mut self) {
		self.allow_null = true;
	}

	pub fn regex(&mut self, regex: Regex) {
		self.validators.push(box RegexValidator::new(regex));
	}

	pub fn validate(&mut self, validator: Box<SingleParamValidator + Send + Sync>) {
		self.validators.push(validator);
	}

	pub fn validate_with(&mut self, validator: fn(&Json) -> Result<(), String>) {
		self.validators.push(box FunctionValidator::new(validator));
	}

	fn process_validations(&self, val: &Json) -> ValicoResult<()> {
		for mut validator in self.validators.iter() {
			try!(validator.validate(val));
		};

		Ok(())
	}

	fn process_nest(&self, val: &mut Json) -> ValicoResult<()> {
		let ref nest = self.nest.as_ref().unwrap();

		if val.is_list() {
			let mut errors = TreeMap::new();
			let list = val.as_list_mut().unwrap();
			for (idx, item) in list.iter_mut().enumerate() {
				if item.is_object() {
					match nest.process(item.as_object_mut().unwrap()) {
						Ok(()) => (),
						Err(err) => { errors.insert(idx.to_string(), err.to_json()); }
					}
				} else {
					errors.insert(idx.to_string(), 
						validation_error(format!("List item {} is not and object", item)).to_json()
					);
				}
			}

			if errors.len() > 0 {
				return Err(errors);
			}
		} else if val.is_object() {
			match nest.process(val.as_object_mut().unwrap()) {
				Ok(()) => (),
				Err(err) => return Err(err)
			};
		}

		Ok(())
	}

	pub fn process(&self, val: &mut Json) -> ValicoResult<Option<Json>> {
		if val.is_null() && self.allow_null { return Ok(None) }

		let mut need_return = false;
		let mut return_value = None;

		let result = {
			let mut val = if self.coercer.is_some() {
				match self.coercer.as_ref().unwrap().coerce(val) {
					Ok(Some(new_value)) => { 
						need_return = true; 
						return_value = Some(new_value); 
						return_value.as_mut().unwrap() 
					},
					Ok(None) => val,
					Err(err) => return Err(err)
				}
			} else {
				val
			};

			if self.nest.is_some() {
				match self.process_nest(val) {
					Ok(()) => (),
					Err(err) => return Err(err)
				};
			}

			self.process_validations(val)
		};
		
		match result {
			Ok(()) => {
				if need_return { Ok(return_value) } else { Ok(None) }
			},
			Err(err) => Err(err)
		}
	}
}

impl<T: ToJson> Param {
	pub fn allow_values(&mut self, values: &[T]) {
		self.validators.push(box AllowedValuesValidator::new(
			values.iter().map(|v| v.to_json()).collect()
		));
	}

	pub fn reject_values(&mut self, values: &[T]) {
		self.validators.push(box RejectedValuesValidator::new(
			values.iter().map(|v| v.to_json()).collect()
		));
	}

	pub fn default(&mut self, default: T) {
		self.default = Some(default.to_json());
	}
}
