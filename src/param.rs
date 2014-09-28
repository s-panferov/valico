
use serialize::json;
use serialize::json::{Json, ToJson};
use regex::Regex;

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

	pub fn process(&self, val: &mut Json) -> ValicoResult<Option<Json>> {
		if val.is_null() && self.allow_null {
			Ok(None)
		} else {
			let result = match self.coercer.as_ref() {
				Some(coercer) => coercer.coerce(val, self.nest.as_ref()),
				None => Ok(None)
			};

			match result {
				Ok(None) => { 
					self.process_validations(val).and(Ok(None))
				},
				Ok(Some(val)) => {
					self.process_validations(&val).and(Ok(Some(val)))
				},
				Err(val) => Err(val)
			}
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
