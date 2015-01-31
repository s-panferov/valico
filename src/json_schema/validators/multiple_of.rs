use serialize::json;

use super::super::errors;
use super::super::scope;

#[allow(missing_copy_implementations)]
pub struct MultipleOf {
    pub number: f64
}

impl super::Validator for MultipleOf {
    fn validate(&self, val: &json::Json, path: &str, strict: bool, _scope: &scope::Scope) -> super::ValidationState {
        let number = strict_process!(val.as_f64(), path, strict, "The value must be a number");

        if number % self.number == 0f64 {
            super::ValidationState::new()
        } else {
            val_error!(
                errors::MultipleOf {
                    path: path.to_string()
                }
            )
        }
    }
}