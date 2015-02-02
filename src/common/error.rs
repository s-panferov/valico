use std::error::{self, Error};
use std::fmt;
use std::any;

pub trait ValicoError: error::Error + fmt::Debug + Send + any::Any {
    fn get_code(&self) -> &str;
    fn get_path(&self) -> &str;
    fn get_title(&self) -> &str;
    fn get_detail(&self) -> Option<&str> { None }
}

pub type ValicoErrors = Vec<Box<ValicoError>>;

mopafy!(ValicoError);

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
            fn get_path(&self) -> &str { self.path.as_slice() }
        }
    };

    ($err:ty, $code:expr, $title:expr, +detail) => {
        impl_basic_err!($err, $code);

        impl $crate::common::error::ValicoError for $err {
            fn get_code(&self) -> &str { $code }
            fn get_title(&self) -> &str { $title }
            fn get_path(&self) -> &str { self.path.as_slice() }
            fn get_detail(&self) -> Option<&str> { Some(self.detail.as_slice()) }
        }
    };

    ($err:ty, $code:expr, $title:expr, +opt_detail) => {
        impl_basic_err!($err, $code);

        impl $crate::common::error::ValicoError for $err {
            fn get_code(&self) -> &str { $code }
            fn get_title(&self) -> &str { $title }
            fn get_path(&self) -> &str { self.path.as_slice() }
            fn get_detail(&self) -> Option<&str> { self.detail.as_ref().map(|s| s.as_slice()) }
        }
    }
}