use serde::{Serialize, Serializer};
use serde_json::{to_value, Value};
use std::fmt;

use super::scope;

#[macro_export]
macro_rules! strict_process {
    ($val:expr, $path:ident, $err:expr) => {{
        let maybe_val = $val;
        if maybe_val.is_none() {
            return val_error!($crate::json_schema::errors::WrongType {
                path: $path.to_string(),
                detail: $err.to_string()
            });
        }

        maybe_val.unwrap()
    }};
}

macro_rules! nonstrict_process {
    ($val:expr, $path:ident) => {{
        let maybe_val = $val;
        if maybe_val.is_none() {
            return $crate::json_schema::validators::ValidationState::new();
        }

        maybe_val.unwrap()
    }};
}

macro_rules! val_error {
    ($err:expr) => {
        $crate::json_schema::validators::ValidationState {
            errors: vec![Box::new($err)],
            missing: vec![],
            replacement: None,
        }
    };
}

pub use self::const_::Const;
pub use self::contains::Contains;
pub use self::dependencies::Dependencies;
pub use self::enum_::Enum;
pub use self::items::Items;
pub use self::maxmin::{ExclusiveMaximum, ExclusiveMinimum, Maximum, Minimum};
pub use self::maxmin_items::{MaxItems, MinItems};
pub use self::maxmin_length::{MaxLength, MinLength};
pub use self::maxmin_properties::{MaxProperties, MinProperties};
pub use self::multiple_of::MultipleOf;
pub use self::not::Not;
pub use self::of::{AllOf, AnyOf, OneOf};
pub use self::pattern::Pattern;
pub use self::properties::Properties;
pub use self::property_names::PropertyNames;
pub use self::ref_::Ref;
pub use self::required::Required;
pub use self::type_::Type;
pub use self::unique_items::UniqueItems;

mod const_;
mod contains;
pub mod dependencies;
mod enum_;
pub mod formats;
pub mod items;
mod maxmin;
mod maxmin_items;
mod maxmin_length;
mod maxmin_properties;
mod multiple_of;
mod not;
mod of;
mod pattern;
pub mod properties;
mod property_names;
mod ref_;
mod required;
pub mod type_;
mod unique_items;

#[derive(Debug)]
pub struct ValidationState {
    pub errors: super::super::common::error::ValicoErrors,
    pub missing: Vec<url::Url>,
    pub replacement: Option<Value>,
}

impl ValidationState {
    pub fn new() -> ValidationState {
        ValidationState {
            errors: vec![],
            missing: vec![],
            replacement: None,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn is_strictly_valid(&self) -> bool {
        self.errors.is_empty() && self.missing.is_empty()
    }

    pub fn append(&mut self, mut second: ValidationState) {
        self.errors.extend(second.errors);
        self.missing.extend(second.missing);
        if second.replacement.is_some() {
            self.replacement = second.replacement.take();
        }
    }

    pub fn replacement_or<'a>(&'a self, data: &'a Value) -> &'a Value {
        self.replacement.as_ref().unwrap_or(data)
    }

    pub fn replace(&mut self, data: &Value, f: impl FnOnce(&mut Value)) {
        if self.replacement.is_none() {
            self.replacement = Some(data.clone());
        }
        f(self.replacement.as_mut().unwrap());
    }
}

impl Serialize for ValidationState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = ::serde_json::Map::new();
        map.insert(
            "errors".to_string(),
            Value::Array(
                self.errors
                    .iter()
                    .map(|err| to_value(err).unwrap())
                    .collect::<Vec<Value>>(),
            ),
        );
        map.insert(
            "missing".to_string(),
            Value::Array(
                self.missing
                    .iter()
                    .map(|url| to_value(&url.to_string()).unwrap())
                    .collect::<Vec<Value>>(),
            ),
        );
        Value::Object(map).serialize(serializer)
    }
}

pub trait Validator {
    fn validate(&self, item: &Value, _: &str, _: &scope::Scope) -> ValidationState;
}

impl fmt::Debug for dyn Validator + 'static + Send + Sync {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("<validator>")
    }
}

pub type BoxedValidator = Box<dyn Validator + 'static + Send + Sync>;
pub type Validators = Vec<BoxedValidator>;

impl<T> Validator for T
where
    T: Fn(&Value, &str, &scope::Scope) -> ValidationState,
{
    fn validate(&self, val: &Value, path: &str, scope: &scope::Scope) -> ValidationState {
        self(val, path, scope)
    }
}
