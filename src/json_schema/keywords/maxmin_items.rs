use rustc_serialize::json;
use std::num::{Float};

use super::super::schema;
use super::super::validators;

kw_minmax_integer!(MaxItems, "maxItems");
kw_minmax_integer!(MinItems, "minItems");

#[cfg(test)] use super::super::scope;
#[cfg(test)] use jsonway;
#[cfg(test)] use rustc_serialize::json::{ToJson};

#[test]
fn validate_max_items() {
    let mut scope = scope::Scope::new();
    let schema = scope.compile_and_return(jsonway::object(|schema| {
        schema.set("maxItems", 5);
    }).unwrap()).ok().unwrap();;

    assert_eq!(schema.validate(&[1,2,3,4].to_json()).is_valid(), true);
    assert_eq!(schema.validate(&[1,2,3,4,5].to_json()).is_valid(), true);
    assert_eq!(schema.validate(&[1,2,3,4,5,6].to_json()).is_valid(), false);
}

#[test]
fn malformed_max_items() {
    let mut scope = scope::Scope::new();

    assert!(scope.compile_and_return(jsonway::object(|schema| {
        schema.set("maxItems", (-1).to_json());
    }).unwrap()).is_err());

    assert!(scope.compile_and_return(jsonway::object(|schema| {
        schema.set("maxItems", "".to_json());
    }).unwrap()).is_err());

    assert!(scope.compile_and_return(jsonway::object(|schema| {
        schema.set("maxItems", (1.1).to_json());
    }).unwrap()).is_err());
}

#[test]
fn validate_min_items() {
    let mut scope = scope::Scope::new();
    let schema = scope.compile_and_return(jsonway::object(|schema| {
        schema.set("minItems", 5);
    }).unwrap()).ok().unwrap();;

    assert_eq!(schema.validate(&[1,2,3,4].to_json()).is_valid(), false);
    assert_eq!(schema.validate(&[1,2,3,4,5].to_json()).is_valid(), true);
    assert_eq!(schema.validate(&[1,2,3,4,5,6].to_json()).is_valid(), true);
}

#[test]
fn malformed_min_items() {
    let mut scope = scope::Scope::new();

    assert!(scope.compile_and_return(jsonway::object(|schema| {
        schema.set("minItems", (-1).to_json());
    }).unwrap()).is_err());

    assert!(scope.compile_and_return(jsonway::object(|schema| {
        schema.set("minItems", "".to_json());
    }).unwrap()).is_err());

    assert!(scope.compile_and_return(jsonway::object(|schema| {
        schema.set("minItems", (1.1).to_json());
    }).unwrap()).is_err());
}