
use serialize::json::{self, ToJson};
use std::collections;
use regex;

use mutable_json::MutableJson;
use builder;
use coercers;
use validation;
use helpers;

pub struct Param {
    pub name: String,
    pub coercer: Option<Box<coercers::Coercer  + Send + Sync>>,
    pub nest: Option<builder::Builder>,
    pub description: Option<String>,
    pub allow_null: bool,
    pub validators: Vec<Box<validation::SingleParamValidator + Send + Sync>>,
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
        self.validators.push(Box::new(validation::RegexValidator::new(regex)));
    }

    pub fn validate(&mut self, validator: Box<validation::SingleParamValidator + Send + Sync>) {
        self.validators.push(validator);
    }

    pub fn validate_with(&mut self, validator: fn(&json::Json) -> Result<(), String>) {
        self.validators.push(Box::new(validation::FunctionValidator::new(validator)));
    }

    fn process_validations(&self, val: &json::Json) -> ::ValicoResult<()> {
        for mut validator in self.validators.iter() {
            try!(validator.validate(val));
        };

        Ok(())
    }

    fn process_nest(&self, val: &mut json::Json) -> ::ValicoResult<()> {
        let ref nest = self.nest.as_ref().unwrap();

        if val.is_array() {
            let mut errors = collections::BTreeMap::new();
            let array = val.as_array_mut().unwrap();
            for (idx, item) in array.iter_mut().enumerate() {
                if item.is_object() {
                    match nest.process(item.as_object_mut().unwrap()) {
                        Ok(()) => (),
                        Err(err) => { errors.insert(idx.to_string(), err.to_json()); }
                    }
                } else {
                    errors.insert(idx.to_string(), 
                        helpers::validation_error(format!("List item {} is not and object", item)).to_json()
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

    pub fn process(&self, val: &mut json::Json) -> ::ValicoResult<Option<json::Json>> {
        if val.is_null() && self.allow_null { return Ok(None) }

        let mut need_return = false;
        let mut return_value = None;

        let result = {
            let val = if self.coercer.is_some() {
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

impl Param {
    pub fn allow_values<T: json::ToJson>(&mut self, values: &[T]) {
        self.validators.push(Box::new(validation::AllowedValuesValidator::new(
            values.iter().map(|v| v.to_json()).collect()
        )));
    }

    pub fn reject_values<T: json::ToJson>(&mut self, values: &[T]) {
        self.validators.push(Box::new(validation::RejectedValuesValidator::new(
            values.iter().map(|v| v.to_json()).collect()
        )));
    }

    pub fn default<T: json::ToJson>(&mut self, default: T) {
        self.default = Some(default.to_json());
    }
}
