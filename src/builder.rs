
use std::collections::TreeMap;
use serialize::json::{JsonObject, ToJson};

use helpers::{has_value, single_validation_error};
use param::Param;

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

use ValicoResult;

pub struct Builder {
	requires: Vec<Param>,
	optional: Vec<Param>
}

impl Builder {

	pub fn new() -> Builder {
		Builder {
			requires: vec![],
			optional: vec![]
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
				errors.insert(name.to_string(), single_validation_error("Field is required".to_string()).to_json());
			}
		}

		if errors.len() == 0 {
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

	// pub fn optional(name: &str, kind: Coeletrcer);
	// pub fn group(name: &str);
	// pub fn mutually_exclusive();
}


