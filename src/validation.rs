
use regex::Regex;
use serialize::json::{Json, JsonObject};

use helpers::{validation_error};
use ValicoResult;

pub trait SingleParamValidator {
    fn validate(&self, &Json) -> ValicoResult<()>;
}

pub trait MultipleParamValidator {
    fn validate(&self, &JsonObject) -> ValicoResult<()>;
}

pub struct AllowedValuesValidator {
    allowed_values: Vec<Json>
}

impl AllowedValuesValidator {
    pub fn new(values: Vec<Json>) -> AllowedValuesValidator {
        AllowedValuesValidator {
            allowed_values: values
        }
    }
}

impl SingleParamValidator for AllowedValuesValidator {
    fn validate(&self, val: &Json) -> ValicoResult<()> {
        let mut matched = false;
        for allowed_value in self.allowed_values.iter() {
            if val == allowed_value { matched = true; }
        }

        if matched {
            Ok(())
        } else {
            Err(validation_error(format!("Value {} is not among allowed list", val)))
        }
    }
}

pub struct RejectedValuesValidator {
    rejected_values: Vec<Json>
}

impl RejectedValuesValidator {
    pub fn new(values: Vec<Json>) -> RejectedValuesValidator {
        RejectedValuesValidator {
            rejected_values: values
        }
    }
}

impl SingleParamValidator for RejectedValuesValidator {
    fn validate(&self, val: &Json) -> ValicoResult<()> {
        let mut matched = false;
        for rejected_value in self.rejected_values.iter() {
            if val == rejected_value { matched = true; }
        }

        if matched {
            Err(validation_error(format!("Value {} is among reject list", val)))
        } else {
            Ok(())
        }
    }
}

pub struct FunctionValidator {
    validator: fn(&Json) -> Result<(), String>
}

impl FunctionValidator {
    pub fn new(validator: fn(&Json) -> Result<(), String>) -> FunctionValidator {
        FunctionValidator {
            validator: validator
        }
    }
}

impl SingleParamValidator for FunctionValidator {
    fn validate(&self, val: &Json) -> ValicoResult<()> {
        let validator = self.validator;
        match validator(val) {
            Ok(()) => Ok(()),
            Err(err) => Err(validation_error(err))
        }
    }
}

pub struct RegexValidator {
    regex: Regex
}

impl RegexValidator {
    pub fn new(regex: Regex) -> RegexValidator {
        RegexValidator {
            regex: regex
        }
    }
}

impl SingleParamValidator for RegexValidator {
    fn validate(&self, val: &Json) -> ValicoResult<()> {
        if val.is_string() {
            if self.regex.is_match(val.as_string().unwrap()) {
                Ok(())
            } else {
                Err(validation_error(format!("Value {} is not match required pattern", val)))
            }
        } else {
            Err(validation_error(format!("Value {} can't be compared with pattern", val)))
        }
    }
}

pub struct MutuallyExclusiveValidator {
    params: Vec<String>
}

impl MutuallyExclusiveValidator {
    pub fn new(params: &[&str]) -> MutuallyExclusiveValidator {
        MutuallyExclusiveValidator {
            params: params.iter().map(|s| s.to_string()).collect()
        }
    }
}

impl MultipleParamValidator for MutuallyExclusiveValidator {
    fn validate(&self, tree: &JsonObject) -> ValicoResult<()> {
        let mut matched = vec![];
        for param in self.params.iter() {
            if tree.contains_key(param) { matched.push(param.clone()); }
        }

        if matched.len() <= 1 {
            Ok(())
        } else {
            Err(validation_error(format!("Fields {} are mutually exclusive", matched)))
        }
    }
}

pub struct ExactlyOneOfValidator {
    params: Vec<String>
}

impl ExactlyOneOfValidator {
    pub fn new(params: &[&str]) -> ExactlyOneOfValidator {
        ExactlyOneOfValidator {
            params: params.iter().map(|s| s.to_string()).collect()
        }
    }
}

impl MultipleParamValidator for ExactlyOneOfValidator {
    fn validate(&self, tree: &JsonObject) -> ValicoResult<()> {
        let mut matched = vec![];
        for param in self.params.iter() {
            if tree.contains_key(param) { matched.push(param.clone()); }
        }

        let len = matched.len();
        if len == 1 {
            Ok(())
        } else if len > 1 {
            Err(validation_error(format!("Exactly one of {} is allowed at one time", matched)))
        } else {
            Err(validation_error(format!("Exactly one of {} must be present", self.params)))
        }
    }
}

pub struct AtLeastOneOfValidator {
    params: Vec<String>
}

impl AtLeastOneOfValidator {
    pub fn new(params: &[&str]) -> AtLeastOneOfValidator {
        AtLeastOneOfValidator {
            params: params.iter().map(|s| s.to_string()).collect()
        }
    }
}

impl MultipleParamValidator for AtLeastOneOfValidator {
    fn validate(&self, tree: &JsonObject) -> ValicoResult<()> {
        let mut matched = vec![];
        for param in self.params.iter() {
            if tree.contains_key(param) { matched.push(param.clone()); }
        }

        let len = matched.len();
        if len >= 1 {
            Ok(())
        } else {
            Err(validation_error(format!("Al least one of {} must be present", self.params)))
        }
    }
}

pub struct FunctionMultipleValidator {
    validator: fn(&JsonObject) -> Result<(), String>
}

impl FunctionMultipleValidator {
    pub fn new(validator: fn(&JsonObject) -> Result<(), String>) -> FunctionMultipleValidator {
        FunctionMultipleValidator {
            validator: validator
        }
    }
}

impl MultipleParamValidator for FunctionMultipleValidator {
    fn validate(&self, val: &JsonObject) -> ValicoResult<()> {
        let validator = self.validator;
        match validator(val) {
            Ok(()) => Ok(()),
            Err(err) => Err(validation_error(err))
        }
    }
}