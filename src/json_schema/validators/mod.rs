use serialize::json;
use std::fmt;
use url;

use super::scope;

macro_rules! strict_process {
    ($val:expr, $path:ident, $strict:expr, $err:expr) => {{
        let maybe_val = $val;
        if maybe_val.is_none() {
            return if !$strict {
                $crate::json_schema::validators::ValidationState::new()
            } else {
                val_error!(
                    $crate::json_schema::errors::WrongType {
                        path: $path.to_string(),
                        detail: $err.to_string()
                    }
                )
            }
        }

        maybe_val.unwrap()
    }}
}

macro_rules! val_error{
    ($err:expr) => (
        $crate::json_schema::validators::ValidationState{
            errors: vec![
                Box::new($err)
            ],
            missing: vec![]
        }
    )
}

pub use self::multiple_of::{MultipleOf};
pub use self::maxmin::{Maximum, Minimum};
pub use self::maxmin_length::{MaxLength, MinLength};
pub use self::pattern::{Pattern};
pub use self::maxmin_items::{MaxItems, MinItems};
pub use self::unique_items::{UniqueItems};
pub use self::items::{Items};
pub use self::maxmin_properties::{MaxProperties, MinProperties};
pub use self::required::{Required};
pub use self::properties::{Properties};
pub use self::dependencies::{Dependencies};
pub use self::enum_::{Enum};

mod multiple_of;
mod maxmin;
mod maxmin_length;
mod pattern;
mod maxmin_items;
mod unique_items;
pub mod items;
mod maxmin_properties;
mod required;
pub mod properties;
pub mod dependencies;
mod enum_;

#[derive(Debug)]
pub struct ValidationState {
    pub errors: super::super::common::error::ValicoErrors,
    pub missing: Vec<url::Url>
}

impl ValidationState {
    pub fn new() -> ValidationState {
        ValidationState {
            errors: vec![],
            missing: vec![]
        }
    }

    pub fn is_valid(&self) -> bool {
        self.errors.len() == 0
    }

    pub fn append(&mut self, second: &mut ValidationState) {
        self.errors.append(&mut second.errors);
        self.missing.append(&mut second.missing);
    }
}

pub trait Validator {
    fn validate(&self, item: &json::Json, &str, bool, &scope::Scope) -> ValidationState;
}

impl fmt::Debug for Validator + 'static {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("<validator>")
    }
}

pub type BoxedValidator = Box<Validator + 'static>;
pub type Validators = Vec<BoxedValidator>;

impl<T> Validator for T where T: Fn(&json::Json, &str, bool, &scope::Scope) -> ValidationState {
    fn validate(&self, val: &json::Json, path: &str, strict: bool, scope: &scope::Scope) -> ValidationState {
        self(val, path, strict, scope)
    }
}
