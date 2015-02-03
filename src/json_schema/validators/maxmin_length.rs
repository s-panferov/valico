use rustc_serialize::json;

use super::super::errors;
use super::super::scope;

#[allow(missing_copy_implementations)]
pub struct MaxLength {
    pub length: u64
}

impl super::Validator for MaxLength {
    fn validate(&self, val: &json::Json, path: &str, strict: bool, _scope: &scope::Scope) -> super::ValidationState {
        let string = strict_process!(val.as_string(), path, strict, "The value must be a string");

        if (string.len() as u64) <= self.length {
            super::ValidationState::new()
        } else {
            val_error!(
                errors::MaxLength {
                    path: path.to_string()
                }
            )
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct MinLength {
    pub length: u64
}

impl super::Validator for MinLength {
    fn validate(&self, val: &json::Json, path: &str, strict: bool, _scope: &scope::Scope) -> super::ValidationState {
        let string = strict_process!(val.as_string(), path, strict, "The value must be a string");

        if (string.len() as u64) >= self.length {
            super::ValidationState::new()
        } else {
            val_error!(
                errors::MinLength {
                    path: path.to_string()
                }
            )
        }
    }
}