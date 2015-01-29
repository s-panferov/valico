use serialize::json;
use std::fmt;
use url;

use super::scope;

macro_rules! strict_process {
    ($val:expr, $path:ident, $strict:expr, $err:expr) => {{
        let maybe_val = $val;
        if maybe_val.is_none() {
            return if !$strict {
                Ok(())
            } else {
                Err(val_error!(
                    $crate::json_schema::errors::WrongType {
                        path: $path.to_string(),
                        detail: $err.to_string()
                    }
                ))
            }
        }

        maybe_val.unwrap()
    }}
}

macro_rules! val_error{
    ($err:expr) => (
        $crate::json_schema::validators::ValidatorError{
            errors: vec![
                Box::new($err)
            ],
            missing: vec![]
        }
    )
}

pub use self::multiple_of::{MultipleOf};
pub use self::maxmin::{Maximum};
pub use self::maxmin::{Minimum};
pub use self::maxmin_length::{MaxLength};
pub use self::maxmin_length::{MinLength};
pub use self::pattern::{Pattern};

mod multiple_of;
mod maxmin;
mod maxmin_length;
mod pattern;

pub struct ValidatorError {
    pub errors: super::super::common::error::ValicoErrors,
    pub missing: Vec<url::Url>
}

impl ValidatorError {
    pub fn append(&mut self, second: &mut ValidatorError) {
        self.errors.append(&mut second.errors);
        self.missing.append(&mut second.missing);
    }
}

pub type ValidatorResult = Result<(), ValidatorError>;

pub trait Validator {
    fn validate(&self, item: &json::Json, &str, bool, &scope::Scope) -> ValidatorResult;
}

impl fmt::Debug for Validator + 'static {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("<validator>")
    }
}

pub type BoxedValidator = Box<Validator + 'static>;
pub type Validators = Vec<BoxedValidator>;

impl<T> Validator for T where T: Fn(&json::Json, &str, bool, &scope::Scope) -> ValidatorResult {
    fn validate(&self, val: &json::Json, path: &str, strict: bool, scope: &scope::Scope) -> ValidatorResult {
        self(val, path, strict, scope)
    }
}
