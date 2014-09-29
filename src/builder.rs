
use std::collections::TreeMap;
use serialize::json::{Json, JsonObject, ToJson};

use helpers::{has_value, validation_error};
use param::Param;

use coercers::{
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

use validation::{
	MultipleParamValidator
};

use ValicoResult;

pub struct Builder {
	requires: Vec<Param>,
	optional: Vec<Param>,
	validators: Vec<Box<MultipleParamValidator + Send + Sync>>
}

impl Builder {

	pub fn new() -> Builder {
		Builder {
			requires: vec![],
			optional: vec![],
			validators: vec![]
		}
	}

	pub fn build(rules: |params: &mut Builder|) -> Builder {
		let mut builder = Builder::new();
		rules(&mut builder);

		builder
	}

	pub fn req_defined(&mut self, name: &str) {
		let params = Param::new(name);
		self.requires.push(params);
	}

	pub fn req_typed(&mut self, name: &str, coercer: Box<Coercer>) {
		let params = Param::new_with_coercer(name, coercer);
		self.requires.push(params);
	}

	pub fn req_nested(&mut self, name: &str, coercer: Box<Coercer>, nest_def: |&mut Builder|) {
		let nest_builder = Builder::build(nest_def);
		let params = Param::new_with_nest(name, coercer, nest_builder);
		self.requires.push(params);
	}

	pub fn req(&mut self, name: &str, param_builder: |&mut Param|) {
		let params = Param::build(name, param_builder);
		self.requires.push(params);
	}

	pub fn opt_defined(&mut self, name: &str) {
		let params = Param::new(name);
		self.optional.push(params);
	}

	pub fn opt_typed(&mut self, name: &str, coercer: Box<Coercer>) {
		let params = Param::new_with_coercer(name, coercer);
		self.optional.push(params);
	}

	pub fn opt_nested(&mut self, name: &str, coercer: Box<Coercer>, nest_def: |&mut Builder|) {
		let nest_builder = Builder::build(nest_def);
		let params = Param::new_with_nest(name, coercer, nest_builder);
		self.optional.push(params);
	}

	pub fn opt(&mut self, name: &str, param_builder: |&mut Param|) {
		let params = Param::build(name, param_builder);
		self.optional.push(params);
	}

	pub fn process(&self, tree: &mut JsonObject) -> ValicoResult<()>  {
		
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
				errors.insert(name.to_string(), validation_error("Field is required".to_string()).to_json());
			}
		}

		for param in self.optional.iter() {
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
			}
		}

		let mut i = 0u;
		for validator in self.validators.iter() {
			match validator.validate(tree) {
				Ok(()) => (),
				Err(err) => {
					errors.insert(format!("$${}", i.to_string()), err.to_json());
					i = i + 1;
				}
			};
		}
	
		if errors.len() == 0 {
			// second pass we need to validate without default values in optionals
			for param in self.optional.iter() {
				let ref name = param.name;
				let present = has_value(tree, name);
				if !present {
					match param.default.as_ref() {
						Some(val) => { tree.insert(name.clone(), val.clone()); },
						None => ()
					};
				}
			}

			Ok(())
		} else {
			Err(errors)
		}
	}

	pub fn i64() -> Box<Coercer + Send + Sync> { box I64Coercer }
	pub fn u64() -> Box<Coercer + Send + Sync> { box U64Coercer }
	pub fn f64() -> Box<Coercer + Send + Sync> { box F64Coercer }
	pub fn string() -> Box<Coercer + Send + Sync> { box StringCoercer }
	pub fn null() -> Box<Coercer + Send + Sync> { box NullCoercer }
	pub fn list() -> Box<Coercer + Send + Sync> { box ListCoercer::new() }
	pub fn list_of(coercer: Box<Coercer + Send + Sync>) -> Box<Coercer + Send + Sync> { box ListCoercer::of_type(coercer) }
	pub fn object() -> Box<Coercer + Send + Sync> { box ObjectCoercer }

}


