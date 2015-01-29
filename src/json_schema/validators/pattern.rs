use regex;
use serialize::json;

use super::super::errors;
use super::super::scope;

#[allow(missing_copy_implementations)]
pub struct Pattern {
    pub regex: regex::Regex
}

impl super::Validator for Pattern {
    fn validate(&self, val: &json::Json, path: &str, strict: bool, _scope: &scope::Scope) -> super::ValidatorResult {
        let string = strict_process!(val.as_string(), path, strict, "The value must be a string");

        if self.regex.is_match(string) {
            Ok(())
        } else {
            Err(val_error!(
                errors::Pattern {
                    path: path.to_string()
                }
            ))
        }
    }
}