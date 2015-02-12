use rustc_serialize::json;
use std::num::{Float};

use super::super::schema;
use super::super::validators;

macro_rules! kw_minmax_integer{
    ($name:ident, $keyword:expr) => {
        #[allow(missing_copy_implementations)]
        pub struct $name;
        impl super::Keyword for $name {
            fn compile(&self, def: &json::Json, ctx: &schema::WalkContext) -> super::KeywordResult {
                let length = keyword_key_exists!(def, $keyword);

                if length.is_number() {
                    let length_val = length.as_f64().unwrap();
                    if length_val >= 0f64 && length_val.fract() == 0f64 {
                        Ok(Some(Box::new(validators::$name {
                            length: length_val as u64
                        })))
                    } else {
                        Err(schema::SchemaError::Malformed {
                            path: ctx.fragment.connect("/"),
                            detail: "The value MUST be a positive integer or zero".to_string()
                        })  
                    }
                } else {
                    Err(schema::SchemaError::Malformed {
                        path: ctx.fragment.connect("/"),
                        detail: "The value MUST be a positive integer or zero".to_string()
                    })
                }
            }
        }
    }
}

kw_minmax_integer!(MaxLength, "maxLength");
kw_minmax_integer!(MinLength, "minLength");

#[cfg(test)] use super::super::scope;
#[cfg(test)] use jsonway;
#[cfg(test)] use super::super::builder;
#[cfg(test)] use rustc_serialize::json::{ToJson};

#[test]
fn validate_max_length() {
    let mut scope = scope::Scope::new();
    let schema = scope.compile_and_return(builder::schema(|s| {
        s.max_length(5u64);
    }).into_json(), true).ok().unwrap();;

    assert_eq!(schema.validate(&"1234".to_json()).is_valid(), true);
    assert_eq!(schema.validate(&"12345".to_json()).is_valid(), true);
    assert_eq!(schema.validate(&"123456".to_json()).is_valid(), false);
}

#[test]
fn malformed_max_length() {
    let mut scope = scope::Scope::new();

    assert!(scope.compile_and_return(jsonway::object(|schema| {
        schema.set("maxLength", (-1).to_json());
    }).unwrap(), true).is_err());

    assert!(scope.compile_and_return(jsonway::object(|schema| {
        schema.set("maxLength", "".to_json());
    }).unwrap(), true).is_err());

    assert!(scope.compile_and_return(jsonway::object(|schema| {
        schema.set("maxLength", (1.1).to_json());
    }).unwrap(), true).is_err());
}

#[test]
fn validate_min_length() {
    let mut scope = scope::Scope::new();
    let schema = scope.compile_and_return(builder::schema(|s| {
        s.min_length(5u64);
    }).into_json(), true).ok().unwrap();;

    assert_eq!(schema.validate(&"1234".to_json()).is_valid(), false);
    assert_eq!(schema.validate(&"12345".to_json()).is_valid(), true);
    assert_eq!(schema.validate(&"123456".to_json()).is_valid(), true);
}

#[test]
fn malformed_min_length() {
    let mut scope = scope::Scope::new();

    assert!(scope.compile_and_return(jsonway::object(|schema| {
        schema.set("minLength", (-1).to_json());
    }).unwrap(), true).is_err());

    assert!(scope.compile_and_return(jsonway::object(|schema| {
        schema.set("minLength", "".to_json());
    }).unwrap(), true).is_err());

    assert!(scope.compile_and_return(jsonway::object(|schema| {
        schema.set("minLength", (1.1).to_json());
    }).unwrap(), true).is_err());
}