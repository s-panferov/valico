use std::error::{Error};
use super::super::common::error::ValicoError;
use rustc_serialize::json;
use rustc_serialize::json::ToJson;
use std::collections;

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct Required {
    pub path: String
}
impl_err!(Required, "required", "This field is required");
impl_to_json!(Required);


#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct WrongType {
    pub path: String,
    pub detail: String
}
impl_err!(WrongType, "wrong_type", "Type of the value is wrong", +detail);
impl_to_json!(WrongType);

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct WrongValue {
    pub path: String,
    pub detail: Option<String>,
}
impl_err!(WrongValue, "wrong_value", "The value is wrong or mailformed", +opt_detail);
impl_to_json!(WrongValue);

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct MutuallyExclusive {
    pub path: String,
    pub detail: Option<String>,
    pub params: Vec<String>
}
impl_err!(MutuallyExclusive, "mutually_exclusive", "The values are mutually exclusive", +opt_detail);
impl_to_json!(MutuallyExclusive, |: err: &MutuallyExclusive, map: &mut collections::BTreeMap<String, json::Json>| {
    map.insert("params".to_string(), err.params.to_json());
});

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct ExactlyOne {
    pub path: String,
    pub detail: Option<String>,
    pub params: Vec<String>
}
impl_err!(ExactlyOne, "exactly_one", "Exacly one of the values must be present", +opt_detail);
impl_to_json!(ExactlyOne, |: err: &ExactlyOne, map: &mut collections::BTreeMap<String, json::Json>| {
    map.insert("params".to_string(), err.params.to_json())
});


#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct AtLeastOne {
    pub path: String,
    pub detail: Option<String>,
    pub params: Vec<String>
}
impl_err!(AtLeastOne, "at_least_one", "At least one of the values must be present", +opt_detail);
impl_to_json!(AtLeastOne, |: err: &AtLeastOne, map: &mut collections::BTreeMap<String, json::Json>| {
    map.insert("params".to_string(), err.params.to_json())
});