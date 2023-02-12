use serde::{Serialize, Serializer};
use serde_json::{to_value, Value};
use std::any::{Any, TypeId};
use std::error::Error;
use std::fmt::Debug;

use crate::json_dsl;
use crate::json_schema;

pub trait GetTypeId: Any {
    fn typeid(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}
impl<T: Any> GetTypeId for T {}

pub fn get_data_ptr<T: ?Sized>(d: *const T) -> *const () {
    d as *const ()
}

pub trait ValicoError: Error + Send + Debug + GetTypeId {
    fn get_code(&self) -> &str;
    fn get_path(&self) -> &str;
    fn get_title(&self) -> &str;
    fn get_detail(&self) -> Option<&str> {
        None
    }
}

impl dyn ValicoError {
    /// Is this `Error` object of type `E`?
    pub fn is<E: ValicoError>(&self) -> bool {
        self.typeid() == TypeId::of::<E>()
    }

    /// If this error is `E`, downcast this error to `E`, by reference.
    pub fn downcast<E: ValicoError>(&self) -> Option<&E> {
        if self.is::<E>() {
            unsafe { Some(&*(get_data_ptr(self) as *const E)) }
        } else {
            None
        }
    }
}

impl Serialize for dyn ValicoError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(err) = self.downcast::<json_dsl::errors::Required>() {
            err.serialize(serializer)
        } else if let Some(err) = self.downcast::<json_dsl::errors::WrongType>() {
            err.serialize(serializer)
        } else if let Some(err) = self.downcast::<json_dsl::errors::WrongValue>() {
            err.serialize(serializer)
        } else if let Some(err) = self.downcast::<json_dsl::errors::MutuallyExclusive>() {
            err.serialize(serializer)
        } else if let Some(err) = self.downcast::<json_dsl::errors::ExactlyOne>() {
            err.serialize(serializer)
        } else if let Some(err) = self.downcast::<json_dsl::errors::AtLeastOne>() {
            err.serialize(serializer)
        } else if let Some(err) = self.downcast::<json_dsl::errors::WrongType>() {
            err.serialize(serializer)
        } else if let Some(err) = self.downcast::<json_schema::errors::WrongType>() {
            err.serialize(serializer)
        } else if let Some(err) = self.downcast::<json_schema::errors::MultipleOf>() {
            err.serialize(serializer)
        } else if let Some(err) = self.downcast::<json_schema::errors::Maximum>() {
            err.serialize(serializer)
        } else if let Some(err) = self.downcast::<json_schema::errors::Minimum>() {
            err.serialize(serializer)
        } else if let Some(err) = self.downcast::<json_schema::errors::MaxLength>() {
            err.serialize(serializer)
        } else if let Some(err) = self.downcast::<json_schema::errors::MinLength>() {
            err.serialize(serializer)
        } else if let Some(err) = self.downcast::<json_schema::errors::Pattern>() {
            err.serialize(serializer)
        } else if let Some(err) = self.downcast::<json_schema::errors::MaxItems>() {
            err.serialize(serializer)
        } else if let Some(err) = self.downcast::<json_schema::errors::MinItems>() {
            err.serialize(serializer)
        } else if let Some(err) = self.downcast::<json_schema::errors::UniqueItems>() {
            err.serialize(serializer)
        } else if let Some(err) = self.downcast::<json_schema::errors::Items>() {
            err.serialize(serializer)
        } else if let Some(err) = self.downcast::<json_schema::errors::MaxProperties>() {
            err.serialize(serializer)
        } else if let Some(err) = self.downcast::<json_schema::errors::MinProperties>() {
            err.serialize(serializer)
        } else if let Some(err) = self.downcast::<json_schema::errors::Required>() {
            err.serialize(serializer)
        } else if let Some(err) = self.downcast::<json_schema::errors::Properties>() {
            err.serialize(serializer)
        } else if let Some(err) = self.downcast::<json_schema::errors::Enum>() {
            err.serialize(serializer)
        } else if let Some(err) = self.downcast::<json_schema::errors::AnyOf>() {
            err.serialize(serializer)
        } else if let Some(err) = self.downcast::<json_schema::errors::OneOf>() {
            err.serialize(serializer)
        } else if let Some(err) = self.downcast::<json_schema::errors::Const>() {
            err.serialize(serializer)
        } else if let Some(err) = self.downcast::<json_schema::errors::Contains>() {
            err.serialize(serializer)
        } else if let Some(err) = self.downcast::<json_schema::errors::ContainsMinMax>() {
            err.serialize(serializer)
        } else if let Some(err) = self.downcast::<json_schema::errors::Not>() {
            err.serialize(serializer)
        } else if let Some(err) = self.downcast::<json_schema::errors::DivergentDefaults>() {
            err.serialize(serializer)
        } else if let Some(err) = self.downcast::<json_schema::errors::Format>() {
            err.serialize(serializer)
        } else if let Some(err) = self.downcast::<json_schema::errors::Unevaluated>() {
            err.serialize(serializer)
        } else {
            let mut map = ::serde_json::Map::new();
            map.insert("code".to_string(), to_value(self.get_code()).unwrap());
            map.insert("title".to_string(), to_value(self.get_title()).unwrap());
            map.insert("path".to_string(), to_value(self.get_path()).unwrap());
            if let Some(ref detail) = self.get_detail() {
                map.insert("detail".to_string(), to_value(detail).unwrap());
            }
            Value::Object(map).serialize(serializer)
        }
    }
}

pub type ValicoErrors = Vec<Box<dyn ValicoError>>;

macro_rules! impl_basic_err {
    ($err:ty, $code:expr) => {
        impl ::std::error::Error for $err {}

        impl ::std::fmt::Display for $err {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(formatter, $code)
            }
        }
    };
}

macro_rules! impl_err {
    ($err:ty, $code:expr, $title:expr) => {
        impl_basic_err!($err, $code);

        impl $crate::common::error::ValicoError for $err {
            fn get_code(&self) -> &str {
                $code
            }
            fn get_title(&self) -> &str {
                $title
            }
            fn get_path(&self) -> &str {
                self.path.as_ref()
            }
        }
    };

    ($err:ty, $code:expr, $title:expr, +detail) => {
        impl_basic_err!($err, $code);

        impl $crate::common::error::ValicoError for $err {
            fn get_code(&self) -> &str {
                $code
            }
            fn get_title(&self) -> &str {
                $title
            }
            fn get_path(&self) -> &str {
                self.path.as_ref()
            }
            fn get_detail(&self) -> Option<&str> {
                Some(self.detail.as_ref())
            }
        }
    };

    ($err:ty, $code:expr, $title:expr, +opt_detail) => {
        impl_basic_err!($err, $code);

        impl $crate::common::error::ValicoError for $err {
            fn get_code(&self) -> &str {
                $code
            }
            fn get_title(&self) -> &str {
                $title
            }
            fn get_path(&self) -> &str {
                self.path.as_ref()
            }
            fn get_detail(&self) -> Option<&str> {
                self.detail.as_ref().map(|s| s.as_ref())
            }
        }
    };
}

macro_rules! impl_serialize {
    ($err:ty) => {
        impl Serialize for $err {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
                let mut map = ::serde_json::Map::new();
                map.insert("code".to_string(), to_value(self.get_code()).unwrap());
                map.insert("title".to_string(), to_value(self.get_title()).unwrap());
                map.insert("path".to_string(), to_value(self.get_path()).unwrap());
                if let Some(ref detail) = self.get_detail() {
                    map.insert("detail".to_string(), to_value(detail).unwrap());
                }
                Value::Object(map).serialize(serializer)
            }
        }
    };
    ($err:ty, $($sp:expr),+) => {
        impl Serialize for $err {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
                let mut map = ::serde_json::Map::new();
                map.insert("code".to_string(), to_value(self.get_code()).unwrap());
                map.insert("title".to_string(), to_value(self.get_title()).unwrap());
                map.insert("path".to_string(), to_value(self.get_path()).unwrap());
                if let Some(ref detail) = self.get_detail() {
                    map.insert("detail".to_string(), to_value(detail).unwrap());
                }
                $({
                    let closure = $sp;
                    closure(self, &mut map);
                })+
                Value::Object(map).serialize(serializer)
            }
        }
    }
}
