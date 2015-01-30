use serialize::json;

use super::super::errors;
use super::super::scope;

#[allow(missing_copy_implementations)]
pub struct MaxItems {
    pub length: u64
}

impl super::Validator for MaxItems {
    fn validate(&self, val: &json::Json, path: &str, strict: bool, _scope: &scope::Scope) -> super::ValidatorResult {
        let array = strict_process!(val.as_array(), path, strict, "The value must be an array");

        if (array.len() as u64) <= self.length {
            Ok(())
        } else {
            Err(val_error!(
                errors::MaxItems {
                    path: path.to_string()
                }
            ))
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct MinItems {
    pub length: u64
}

impl super::Validator for MinItems {
    fn validate(&self, val: &json::Json, path: &str, strict: bool, _scope: &scope::Scope) -> super::ValidatorResult {
        let array = strict_process!(val.as_array(), path, strict, "The value must be an array");

        if (array.len() as u64) >= self.length {
            Ok(())
        } else {
            Err(val_error!(
                errors::MinItems {
                    path: path.to_string()
                }
            ))
        }
    }
}