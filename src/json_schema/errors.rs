use std::error::{Error};

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct WrongType {
    pub path: String,
    pub detail: String
}
impl_err!(WrongType, "wrong_type", "Type of the value is wrong", +detail);

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct MultipleOf {
    pub path: String
}
impl_err!(MultipleOf, "multiple_of", "Wrong number of the value");

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct Maximum {
    pub path: String
}
impl_err!(Maximum, "maximum", "Maximum condition is not met");

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct Minimum {
    pub path: String
}
impl_err!(Minimum, "minimum", "Minimum condition is not met");

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct MaxLength {
    pub path: String
}
impl_err!(MaxLength, "max_length", "MaxLength condition is not met");