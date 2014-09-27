
use std::collections::TreeMap;
use serialize::json::{JsonObject, ToJson};

use helpers::{has_value, single_validation_error};
use param::Param;
use coercers::Coercer;
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

	pub fn from_function(rules: |params: &mut Builder|) -> Builder {
		let mut builder = Builder::new();
		rules(&mut builder);

		builder
	}

	pub fn req(&mut self, name: &str) {
		let params = Param::new(name);
		self.requires.push(params);
	}

	pub fn req_type(&mut self, name: &str, coercer: Box<Coercer>) {
		let params = Param::new_with_coercer(name, coercer);
		self.requires.push(params);
	}

	pub fn req_nest(&mut self, name: &str, coercer: Box<Coercer>, extra: |params: &mut Builder|) {
		let extra_builder = Builder::from_function(extra);
		let params = Param::new_with_extra(name, coercer, extra_builder);
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

	// pub fn optional(name: &str, kind: Coeletrcer);
	// pub fn group(name: &str);
	// pub fn mutually_exclusive();
}


