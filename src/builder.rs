
use std::collections;
use serialize::json::{self, ToJson};

use helpers;
use param;

use coercers;
use validation;

use ValicoResult;

pub struct Builder {
    requires: Vec<param::Param>,
    optional: Vec<param::Param>,
    validators: Vec<Box<validation::MultipleParamValidator + Send + Sync>>
}

unsafe impl Send for Builder { }

impl Builder {

    pub fn new() -> Builder {
        Builder {
            requires: vec![],
            optional: vec![],
            validators: vec![]
        }
    }

    pub fn build<F>(rules: F) -> Builder where F: Fn(&mut Builder) {
        let mut builder = Builder::new();
        rules(&mut builder);

        builder
    }

    pub fn req_defined(&mut self, name: &str) {
        let params = param::Param::new(name);
        self.requires.push(params);
    }

    pub fn req_typed(&mut self, name: &str, coercer: Box<coercers::Coercer + Send + Sync>) {
        let params = param::Param::new_with_coercer(name, coercer);
        self.requires.push(params);
    }

    pub fn req_nested<F>(&mut self, name: &str, coercer: Box<coercers::Coercer + Send + Sync>, nest_def: F) where F: Fn(&mut Builder) {
        let nest_builder = Builder::build(nest_def);
        let params = param::Param::new_with_nest(name, coercer, nest_builder);
        self.requires.push(params);
    }

    pub fn req<F>(&mut self, name: &str, param_builder: F) where F: Fn(&mut param::Param) {
        let params = param::Param::build(name, param_builder);
        self.requires.push(params);
    }

    pub fn opt_defined(&mut self, name: &str) {
        let params = param::Param::new(name);
        self.optional.push(params);
    }

    pub fn opt_typed(&mut self, name: &str, coercer: Box<coercers::Coercer + Send + Sync>) {
        let params = param::Param::new_with_coercer(name, coercer);
        self.optional.push(params);
    }

    pub fn opt_nested<F>(&mut self, name: &str, coercer: Box<coercers::Coercer + Send + Sync>, nest_def: F) where F: Fn(&mut Builder) {
        let nest_builder = Builder::build(nest_def);
        let params = param::Param::new_with_nest(name, coercer, nest_builder);
        self.optional.push(params);
    }

    pub fn opt<F>(&mut self, name: &str, param_builder: F) where F: Fn(&mut param::Param) {
        let params = param::Param::build(name, param_builder);
        self.optional.push(params);
    }

    pub fn validate(&mut self, validator: Box<validation::MutuallyExclusiveValidator>) {
        self.validators.push(validator);
    }

    pub fn validate_with(&mut self, validator: fn(&json::Object) -> Result<(), String>) {
        self.validators.push(Box::new(validation::FunctionMultipleValidator::new(validator)));
    }

    pub fn mutually_exclusive(&mut self, params: &[&str]) {
        let validator = Box::new(validation::MutuallyExclusiveValidator::new(params));
        self.validators.push(validator);
    }

    pub fn exactly_one_of(&mut self, params: &[&str]) {
        let validator = Box::new(validation::ExactlyOneOfValidator::new(params));
        self.validators.push(validator);
    }

    pub fn at_least_one_of(&mut self, params: &[&str]) {
        let validator = Box::new(validation::AtLeastOneOfValidator::new(params));
        self.validators.push(validator);
    }

    pub fn process(&self, tree: &mut json::Object) -> ValicoResult<()>  {
        
        let mut errors = collections::BTreeMap::new();

        for param in self.requires.iter() {
            let ref name = param.name;
            let present = helpers::has_value(tree, name);
            if present {
                match param.process(tree.get_mut(name).unwrap()) {
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
                errors.insert(name.to_string(), helpers::validation_error("Field is required".to_string()).to_json());
            }
        }

        for param in self.optional.iter() {
            let ref name = param.name;
            let present = helpers::has_value(tree, name);
            if present {
                match param.process(tree.get_mut(name).unwrap()) {
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

        let mut i = 0us;
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
                let present = helpers::has_value(tree, name);
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
}


