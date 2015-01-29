use serialize::json;

use super::super::schema;
use super::super::validators;

#[allow(missing_copy_implementations)]
pub struct Maximum;
impl super::Keyword for Maximum {
    fn compile(&self, def: &json::Json, ctx: &schema::WalkContext) -> super::KeywordResult {
        let maximum = def.find("maximum");
        let exclusive_maximum = def.find("exclusiveMaximum");

        if exclusive_maximum.is_some() {
            if !maximum.is_some() {
                return Err(schema::SchemaError::Malformed {
                    path: ctx.fragment.connect("/"),
                    detail: "`exclusiveMaximum` can't go without `maximum`".to_string()
                })
            }
        }

        if maximum.is_some() {
            let maximum_val = maximum.unwrap();
            if maximum_val.is_number() {
                let maximum_val = maximum_val.as_f64().unwrap();
                Ok(Some(Box::new(validators::Maximum {
                    number: maximum_val,
                    exclusive: exclusive_maximum.is_some() && 
                               try!(exclusive_maximum.unwrap()
                                    .as_boolean()
                                    .ok_or_else(|| 
                                        schema::SchemaError::Malformed {
                                            path: ctx.fragment.connect("/"),
                                            detail: "`exclusiveMaximum` must be boolean".to_string()
                                        }
                                    ))
                })))
            } else {
                Err(schema::SchemaError::Malformed {
                    path: ctx.fragment.connect("/"),
                    detail: "the maximum value must be a number".to_string()
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
        schema.set("maximum", 10);
    }).unwrap()).ok().unwrap();

    assert_eq!(schema.validate(&9.to_json()).valid, true);
    assert_eq!(schema.validate(&10.to_json()).valid, true);
    assert_eq!(schema.validate(&11.to_json()).valid, false);
}

#[test]
fn validate_exclusive() {
    let mut scope = scope::Scope::new();
    let schema = scope.compile_and_return(jsonway::object(|schema| {
        schema.set("maximum", 10);
        schema.set("exclusiveMaximum", true);
    }).unwrap()).ok().unwrap();

    assert_eq!(schema.validate(&9.to_json()).valid, true);
    assert_eq!(schema.validate(&10.to_json()).valid, false);
    assert_eq!(schema.validate(&11.to_json()).valid, false);
}

#[test]
fn mailformed_maximum() {
    let mut scope = scope::Scope::new();
    
    assert!(scope.compile_and_return(jsonway::object(|schema| {
        schema.set("maximum", true);
    }).unwrap()).is_err());
}

#[test]
fn mailformed_exclusive_maximum() {
    let mut scope = scope::Scope::new();
    
    assert!(scope.compile_and_return(jsonway::object(|schema| {
        schema.set("exclusiveMaximum", true);
    }).unwrap()).is_err());

    assert!(scope.compile_and_return(jsonway::object(|schema| {
        schema.set("maximum", 10);
        schema.set("exclusiveMaximum", "".to_string());
    }).unwrap()).is_err());
}
