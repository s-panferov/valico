
use regex;
use serialize::json;
use helpers;

pub trait SingleParamValidator {
    fn validate(&self, &json::Json) -> ::ValicoResult<()>;
}

pub trait MultipleParamValidator {
    fn validate(&self, &json::Object) -> ::ValicoResult<()>;
}

pub struct AllowedValuesValidator {
    allowed_values: Vec<json::Json>
}

impl AllowedValuesValidator {
    pub fn new(values: Vec<json::Json>) -> AllowedValuesValidator {
        AllowedValuesValidator {
            allowed_values: values
        }
    }
}

impl SingleParamValidator for AllowedValuesValidator {
    fn validate(&self, val: &json::Json) -> ::ValicoResult<()> {
        let mut matched = false;
        for allowed_value in self.allowed_values.iter() {
            if val == allowed_value { matched = true; }
        }

        if matched {
            Ok(())
        } else {
            Err(helpers::validation_error(format!("Value {} is not among allowed list", val)))
        }
    }
}

pub struct RejectedValuesValidator {
    rejected_values: Vec<json::Json>
}

impl RejectedValuesValidator {
    pub fn new(values: Vec<json::Json>) -> RejectedValuesValidator {
        RejectedValuesValidator {
            rejected_values: values
        }
    }
}

impl SingleParamValidator for RejectedValuesValidator {
    fn validate(&self, val: &json::Json) -> ::ValicoResult<()> {
        let mut matched = false;
        for rejected_value in self.rejected_values.iter() {
            if val == rejected_value { matched = true; }
        }

        if matched {
            Err(helpers::validation_error(format!("Value {} is among reject list", val)))
        } else {
            Ok(())
        }
    }
}

pub struct FunctionValidator {
    validator: fn(&json::Json) -> Result<(), String>
}

impl FunctionValidator {
    pub fn new(validator: fn(&json::Json) -> Result<(), String>) -> FunctionValidator {
        FunctionValidator {
            validator: validator
        }
    }
}

impl SingleParamValidator for FunctionValidator {
    fn validate(&self, val: &json::Json) -> ::ValicoResult<()> {
        let validator = self.validator;
        match validator(val) {
            Ok(()) => Ok(()),
            Err(err) => Err(helpers::validation_error(err))
        }
    }
}

pub struct RegexValidator {
    regex: regex::Regex
}

impl RegexValidator {
    pub fn new(regex: regex::Regex) -> RegexValidator {
        RegexValidator {
            regex: regex
        }
    }
}

impl SingleParamValidator for RegexValidator {
    fn validate(&self, val: &json::Json) -> ::ValicoResult<()> {
        if val.is_string() {
            if self.regex.is_match(val.as_string().unwrap()) {
                Ok(())
            } else {
                Err(helpers::validation_error(format!("Value {} is not match required pattern", val)))
            }
        } else {
            Err(helpers::validation_error(format!("Value {} can't be compared with pattern", val)))
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
    fn validate(&self, tree: &json::Object) -> ::ValicoResult<()> {
        let mut matched = vec![];
        for param in self.params.iter() {
            if tree.contains_key(param) { matched.push(param.clone()); }
        }

        if matched.len() <= 1 {
            Ok(())
        } else {
            Err(helpers::validation_error(format!("Fields {:?} are mutually exclusive", matched)))
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
    fn validate(&self, tree: &json::Object) -> ::ValicoResult<()> {
        let mut matched = vec![];
        for param in self.params.iter() {
            if tree.contains_key(param) { matched.push(param.clone()); }
        }

        let len = matched.len();
        if len == 1 {
            Ok(())
        } else if len > 1 {
            Err(helpers::validation_error(format!("Exactly one of {:?} is allowed at one time", matched)))
        } else {
            Err(helpers::validation_error(format!("Exactly one of {:?} must be present", self.params)))
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
    fn validate(&self, tree: &json::Object) -> ::ValicoResult<()> {
        let mut matched = vec![];
        for param in self.params.iter() {
            if tree.contains_key(param) { matched.push(param.clone()); }
        }

        let len = matched.len();
        if len >= 1 {
            Ok(())
        } else {
            Err(helpers::validation_error(format!("Al least one of {:?} must be present", self.params)))
        }
    }
}

pub struct FunctionMultipleValidator {
    validator: fn(&json::Object) -> Result<(), String>
}

impl FunctionMultipleValidator {
    pub fn new(validator: fn(&json::Object) -> Result<(), String>) -> FunctionMultipleValidator {
        FunctionMultipleValidator {
            validator: validator
        }
    }
}

impl MultipleParamValidator for FunctionMultipleValidator {
    fn validate(&self, val: &json::Object) -> ::ValicoResult<()> {
        let validator = self.validator;
        match validator(val) {
            Ok(()) => Ok(()),
            Err(err) => Err(helpers::validation_error(err))
        }
    }
}