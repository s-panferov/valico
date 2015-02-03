use rustc_serialize::json::{self};

use mutable_json::{MutableJson};
use super::helpers;
use super::param;
use super::coercers;
use super::validators;
use super::errors;

pub struct Builder {
    requires: Vec<param::Param>,
    optional: Vec<param::Param>,
    validators: validators::Validators
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

    pub fn build<F>(rules: F) -> Builder where F: FnOnce(&mut Builder) {
        let mut builder = Builder::new();
        rules(&mut builder);

        builder
    }

    pub fn get_required(&self) -> &Vec<param::Param> {
        return &self.requires;
    }

    pub fn get_optional(&self) -> &Vec<param::Param> {
        return &self.optional;
    }

    pub fn get_validators(&self) -> &validators::Validators {
        return &self.validators;
    }

    pub fn req_defined(&mut self, name: &str) {
        let params = param::Param::new(name);
        self.requires.push(params);
    }

    pub fn req_typed(&mut self, name: &str, coercer: Box<coercers::Coercer + Send + Sync>) {
        let params = param::Param::new_with_coercer(name, coercer);
        self.requires.push(params);
    }

    pub fn req_nested<F>(&mut self, name: &str, coercer: Box<coercers::Coercer + Send + Sync>, nest_def: F) where F: FnOnce(&mut Builder) {
        let nest_builder = Builder::build(nest_def);
        let params = param::Param::new_with_nest(name, coercer, nest_builder);
        self.requires.push(params);
    }

    pub fn req<F>(&mut self, name: &str, param_builder: F) where F: FnOnce(&mut param::Param) {
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

    pub fn opt_nested<F>(&mut self, name: &str, coercer: Box<coercers::Coercer + Send + Sync>, nest_def: F) where F: FnOnce(&mut Builder) {
        let nest_builder = Builder::build(nest_def);
        let params = param::Param::new_with_nest(name, coercer, nest_builder);
        self.optional.push(params);
    }

    pub fn opt<F>(&mut self, name: &str, param_builder: F) where F: FnOnce(&mut param::Param) {
        let params = param::Param::build(name, param_builder);
        self.optional.push(params);
    }

    pub fn validate(&mut self, validator: Box<validators::Validator + 'static>) {
        self.validators.push(validator);
    }

    pub fn validate_with<F>(&mut self, validator: F) where F: Fn(&json::Json, &str, bool) -> validators::ValidatorResult + Send+Sync {
        self.validators.push(Box::new(validator));
    }

    pub fn mutually_exclusive(&mut self, params: &[&str]) {
        let validator = Box::new(validators::MutuallyExclusive::new(params));
        self.validators.push(validator);
    }

    pub fn exactly_one_of(&mut self, params: &[&str]) {
        let validator = Box::new(validators::ExactlyOneOf::new(params));
        self.validators.push(validator);
    }

    pub fn at_least_one_of(&mut self, params: &[&str]) {
        let validator = Box::new(validators::AtLeastOneOf::new(params));
        self.validators.push(validator);
    }

    pub fn process(&self, val: &mut json::Json) -> super::DslResult<()> {
        self.process_path(val, "")
    }

    pub fn process_path(&self, val: &mut json::Json, path: &str) -> super::DslResult<()>  {
        
        let mut errors = vec![];

        {
            let object = val.as_object_mut().expect("DSL works only with objects now");
            for param in self.requires.iter() {
                let ref name = param.name;
                let present = helpers::has_value(object, name);
                let param_path = [path, name.as_slice()].connect("/");
                if present {
                    match param.process(object.get_mut(name).unwrap(), param_path.as_slice()) {
                        Ok(result) => { 
                            match result {
                                Some(new_value) => { object.insert(name.clone(), new_value); },
                                None => ()
                            }
                        },
                        Err(mut err) => {
                            errors.append(&mut err);
                        }
                    }
                } else {
                    errors.push(Box::new(errors::Required {
                        path: param_path.clone()
                    }))
                }
            }

            for param in self.optional.iter() {
                let ref name = param.name;
                let present = helpers::has_value(object, name);
                let param_path = [path, name.as_slice()].connect("/");
                if present {
                    match param.process(object.get_mut(name).unwrap(), param_path.as_slice()) {
                        Ok(result) => { 
                            match result {
                                Some(new_value) => { object.insert(name.clone(), new_value); },
                                None => ()
                            }
                        },
                        Err(mut err) => {
                            errors.append(&mut err)
                        }
                    }
                }
            }
        }

        let path = if path == "" {
            "/"
        } else {
            path
        };

        for validator in self.validators.iter() {
            match validator.validate(val, path, true) {
                Ok(()) => (),
                Err(mut err) => {
                    errors.append(&mut err);
                }
            };
        }

        {
            let object = val.as_object_mut().expect("DSL works only with objects now");
            if errors.len() == 0 {
                // second pass we need to validate without default values in optionals
                for param in self.optional.iter() {
                    let ref name = param.name;
                    let present = helpers::has_value(object, name);
                    if !present {
                        match param.default.as_ref() {
                            Some(val) => { object.insert(name.clone(), val.clone()); },
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
}


