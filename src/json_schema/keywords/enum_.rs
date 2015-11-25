use rustc_serialize::json;

use super::super::schema;
use super::super::validators;

#[allow(missing_copy_implementations)]
pub struct Enum;
impl super::Keyword for Enum {
    fn compile(&self, def: &json::Json, ctx: &schema::WalkContext) -> super::KeywordResult {
        let enum_ = keyword_key_exists!(def, "enum");

        if enum_.is_array() {
            let enum_ = enum_.as_array().unwrap();

            if enum_.len() == 0 {
                return Err(schema::SchemaError::Malformed {
                    path: ctx.fragment.join("/"),
                    detail: "This array MUST have at least one element.".to_string()
                })
            }

            Ok(Some(Box::new(validators::Enum {
                items: enum_.clone()
            })))
        } else {
            Err(schema::SchemaError::Malformed {
                path: ctx.fragment.join("/"),
                detail: "The value of this keyword MUST be an array.".to_string()
            })
        }
    }
}

#[cfg(test)] use super::super::scope;
#[cfg(test)] use jsonway;
#[cfg(test)] use super::super::builder;
#[cfg(test)] use rustc_serialize::json::{ToJson};

#[test]
fn validate() {
    let mut scope = scope::Scope::new();
    let schema = scope.compile_and_return(builder::schema(|s| {
        s.enum_(|items| {
            items.push("prop1".to_string());
            items.push("prop2".to_string());
        })
    }).to_json(), true).ok().unwrap();

    assert_eq!(schema.validate(&"prop1".to_json()).is_valid(), true);
    assert_eq!(schema.validate(&"prop2".to_json()).is_valid(), true);
    assert_eq!(schema.validate(&"prop3".to_json()).is_valid(), false);
    assert_eq!(schema.validate(&1.to_json()).is_valid(), false);
}

#[test]
fn malformed() {
    let mut scope = scope::Scope::new();

    assert!(scope.compile_and_return(jsonway::object(|schema| {
        schema.array("enum", |_| {});
    }).unwrap(), true).is_err());

    assert!(scope.compile_and_return(jsonway::object(|schema| {
        schema.object("enum", |_| {});
    }).unwrap(), true).is_err());
}