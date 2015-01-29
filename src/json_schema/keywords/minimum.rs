use serialize::json;

use super::super::schema;
use super::super::validators;

#[allow(missing_copy_implementations)]
pub struct Minimum;
impl super::Keyword for Minimum {
    fn compile(&self, def: &json::Json, ctx: &schema::WalkContext) -> super::KeywordResult {
        let minumum = def.find("minumum");
        let exclusive_minumum = def.find("exclusiveMinimum");

        if exclusive_minumum.is_some() {
            if !minumum.is_some() {
                return Err(schema::SchemaError::Malformed {
                    path: ctx.fragment.connect("/"),
                    detail: "`exclusiveMinimum` can't go without `minumum`".to_string()
                })
            }
        }

        if minumum.is_some() {
            let minumum_val = minumum.unwrap();
            if minumum_val.is_number() {
                let minumum_val = minumum_val.as_f64().unwrap();
                Ok(Some(Box::new(validators::Minimum {
                    number: minumum_val,
                    exclusive: exclusive_minumum.is_some() &&
                               try!(exclusive_minumum.unwrap()
                                    .as_boolean()
                                    .ok_or_else(||
                                        schema::SchemaError::Malformed {
                                            path: ctx.fragment.connect("/"),
                                            detail: "`exclusiveMinimum` must be boolean".to_string()
                                        }
                                    ))
                })))
            } else {
                Err(schema::SchemaError::Malformed {
                    path: ctx.fragment.connect("/"),
                    detail: "the minumum value must be a number".to_string()
                })
            }
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)] use super::super::scope;
#[cfg(test)] use jsonway;
#[cfg(test)] use serialize::json::{ToJson};

#[test]
fn validate() {
    let mut scope = scope::Scope::new();
    let schema = scope.compile_and_return(jsonway::object(|schema| {
        schema.set("minumum", 10);
    }).unwrap()).ok().unwrap();

    assert_eq!(schema.validate(&9.to_json()).valid, false);
    assert_eq!(schema.validate(&10.to_json()).valid, true);
    assert_eq!(schema.validate(&11.to_json()).valid, true);
}

#[test]
fn validate_exclusive() {
    let mut scope = scope::Scope::new();
    let schema = scope.compile_and_return(jsonway::object(|schema| {
        schema.set("minumum", 10);
        schema.set("exclusiveMinimum", true);
    }).unwrap()).ok().unwrap();

    assert_eq!(schema.validate(&9.to_json()).valid, false);
    assert_eq!(schema.validate(&10.to_json()).valid, false);
    assert_eq!(schema.validate(&11.to_json()).valid, true);
}

#[test]
fn mailformed_minumum() {
    let mut scope = scope::Scope::new();

    assert!(scope.compile_and_return(jsonway::object(|schema| {
        schema.set("minumum", true);
    }).unwrap()).is_err());
}

#[test]
fn mailformed_exclusive_minumum() {
    let mut scope = scope::Scope::new();

    assert!(scope.compile_and_return(jsonway::object(|schema| {
        schema.set("exclusiveMinimum", true);
    }).unwrap()).is_err());

    assert!(scope.compile_and_return(jsonway::object(|schema| {
        schema.set("minumum", 10);
        schema.set("exclusiveMinimum", "".to_string());
    }).unwrap()).is_err());
}
