use rustc_serialize::json;
use std::mem;
use std::fmt::Debug;
use std::any::TypeId;
use std::error::Error as StdError;
use typeable::Typeable;
use traitobject;

pub trait Error : Debug + Send + Typeable + StdError {}

impl<S: StdError + Debug + Send + Typeable> Error for S {}

pub trait ValicoError : Error + json::ToJson {
    fn get_code(&self) -> &str;
    fn get_path(&self) -> &str;
    fn get_title(&self) -> &str;
    fn get_detail(&self) -> Option<&str> { None }
}

impl json::ToJson for Box<ValicoError> {
    fn to_json(&self) -> json::Json {
        (**self).to_json()
    }
}
 
impl ValicoError {
    pub fn is<E: Error>(&self) -> bool {
        self.get_type() == TypeId::of::<E>()
    }

    pub fn downcast<E: Error>(&self) -> Option<&E> {
        if self.is::<E>() {
            unsafe { Some(mem::transmute(traitobject::data(self))) }
        } else {
            None
        }
    }
}

pub type ValicoErrors = Vec<Box<ValicoError>>;

macro_rules! impl_basic_err {
    ($err:ty, $code:expr) => {
        impl ::std::error::Error for $err {
            fn description(&self) -> &str {
                $code
            }
        }

        impl ::std::fmt::Display for $err {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.description().fmt(formatter)
            }
        }
    }
}

macro_rules! impl_err {
    ($err:ty, $code:expr, $title:expr) => {
        impl_basic_err!($err, $code);

        impl $crate::common::error::ValicoError for $err {
            fn get_code(&self) -> &str { $code }
            fn get_title(&self) -> &str { $title }
            fn get_path(&self) -> &str { self.path.as_ref() }
        }
    };

    ($err:ty, $code:expr, $title:expr, +detail) => {
        impl_basic_err!($err, $code);

        impl $crate::common::error::ValicoError for $err {
            fn get_code(&self) -> &str { $code }
            fn get_title(&self) -> &str { $title }
            fn get_path(&self) -> &str { self.path.as_ref() }
            fn get_detail(&self) -> Option<&str> { Some(self.detail.as_ref()) }
        }
    };

    ($err:ty, $code:expr, $title:expr, +opt_detail) => {
        impl_basic_err!($err, $code);

        impl $crate::common::error::ValicoError for $err {
            fn get_code(&self) -> &str { $code }
            fn get_title(&self) -> &str { $title }
            fn get_path(&self) -> &str { self.path.as_ref() }
            fn get_detail(&self) -> Option<&str> { self.detail.as_ref().map(|s| s.as_ref()) }
        }
    }
}

macro_rules! impl_to_json{
    ($err:ty) => {
        impl json::ToJson for $err {
            fn to_json(&self) -> json::Json {
                let mut map = ::std::collections::BTreeMap::new();
                map.insert("code".to_string(), self.get_code().to_json());
                map.insert("title".to_string(), self.get_title().to_json());
                map.insert("path".to_string(), self.get_path().to_json());
                match self.get_detail() {
                    Some(ref detail) => { map.insert("detail".to_string(), detail.to_json()); },
                    None => ()
                }
                json::Json::Object(map)
            }
        }
    };
    ($err:ty, $($sp:expr),+) => {
        impl json::ToJson for $err {
            fn to_json(&self) -> json::Json {
                let mut map = ::std::collections::BTreeMap::new();
                map.insert("code".to_string(), self.get_code().to_json());
                map.insert("title".to_string(), self.get_title().to_json());
                map.insert("path".to_string(), self.get_path().to_json());
                match self.get_detail() {
                    Some(ref detail) => { map.insert("detail".to_string(), detail.to_json()); },
                    None => ()
                }
                $({
                    let closure = $sp;
                    closure(self, &mut map);
                })+
                json::Json::Object(map)
            }
        }
    }
}