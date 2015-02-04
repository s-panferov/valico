use rustc_serialize::json::{self, ToJson};
use regex;

use mutable_json::MutableJson;
use super::builder;
use super::coercers;
use super::validators;
use super::errors;

pub struct Param {
    pub name: String,
    pub coercer: Option<Box<coercers::Coercer  + Send + Sync>>,
    pub nest: Option<builder::Builder>,
    pub description: Option<String>,
    pub allow_null: bool,
    pub validators: validators::Validators,
    pub default: Option<json::Json>
}

unsafe impl Send for Param { }

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

    pub fn new_with_coercer(name: &str, coercer: Box<coercers::Coercer  + Send + Sync>) -> Param {
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

    pub fn new_with_nest(name: &str, coercer: Box<coercers::Coercer + Send + Sync>, nest: builder::Builder) -> Param {
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

    pub fn build<F>(name: &str, build_def: F) -> Param where F: FnOnce(&mut Param) {
        let mut param = Param::new(name);
        build_def(&mut param);

        param
    }

    pub fn desc(&mut self, description: &str) {
        self.description = Some(description.to_string());
    }

    pub fn coerce(&mut self, coercer: Box<coercers::Coercer + Send + Sync>) {
        self.coercer = Some(coercer);
    }

    pub fn nest<F>(&mut self, nest_def: F) where F: FnOnce(&mut builder::Builder) -> () {
        self.nest = Some(builder::Builder::build(nest_def));
    }

    pub fn allow_null(&mut self) {
        self.allow_null = true;
    }

    pub fn regex(&mut self, regex: regex::Regex) {
        self.validators.push(Box::new(regex));
    }

    pub fn validate(&mut self, validator: Box<validators::Validator + 'static + Send + Sync>) {
        self.validators.push(validator);
    }

    pub fn validate_with<F>(&mut self, validator: F) where F: Fn(&json::Json, &str, bool) -> super::DslResult<()> + Send+Sync {
        self.validators.push(Box::new(validator));
    }

    fn process_validatorss(&self, val: &json::Json, path: &str) -> super::DslResult<()> {
        for validator in self.validators.iter() {
            try!(validator.validate(val, path, true));
        };

        Ok(())
    }

    fn process_nest(&self, val: &mut json::Json, path: &str) -> super::DslResult<()> {
        let ref nest = self.nest.as_ref().unwrap();

        if val.is_array() {
            let mut errors = vec![];
            let array = val.as_array_mut().unwrap();
            for (idx, item) in array.iter_mut().enumerate() {
                let item_path = [path, idx.to_string().as_slice()].connect("/");
                if item.is_object() {
                    match nest.process_path(item, item_path.as_slice()) {
                        Ok(()) => (),
                        Err(mut err) => errors.append(&mut err)
                    }
                } else {
                    errors.push(
                        Box::new(errors::WrongType {
                            path: item_path.to_string(),
                            detail: "List value is not and object".to_string()
                        })
                    )
                }
            }

            if errors.len() > 0 {
                return Err(errors);
            }
        } else if val.is_object() {
            match nest.process_path(val, path) {
                Ok(()) => (),
                Err(err) => return Err(err)
            };
        }

        Ok(())
    }

    pub fn process(&self, val: &mut json::Json, path: &str) -> super::DslResult<Option<json::Json>> {
        if val.is_null() && self.allow_null { return Ok(None) }

        let mut need_return = false;
        let mut return_value = None;

        let result = {
            let val = if self.coercer.is_some() {
                match self.coercer.as_ref().unwrap().coerce(val, path) {
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
                match self.process_nest(val, path) {
                    Ok(()) => (),
                    Err(err) => return Err(err)
                };
            }

            self.process_validatorss(val, path)
        };
        
        match result {
            Ok(()) => {
                if need_return { Ok(return_value) } else { Ok(None) }
            },
            Err(err) => Err(err)
        }
    }
}

impl Param {
    pub fn allow_values<T: json::ToJson>(&mut self, values: &[T]) {
        self.validators.push(Box::new(validators::AllowedValues::new(
            values.iter().map(|v| v.to_json()).collect()
        )));
    }

    pub fn reject_values<T: json::ToJson>(&mut self, values: &[T]) {
        self.validators.push(Box::new(validators::RejectedValues::new(
            values.iter().map(|v| v.to_json()).collect()
        )));
    }

    pub fn default<T: json::ToJson>(&mut self, default: T) {
        self.default = Some(default.to_json());
    }
}
