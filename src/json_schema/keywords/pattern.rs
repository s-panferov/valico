use rustc_serialize::json;
use regex;

use super::super::schema;
use super::super::validators;

#[allow(missing_copy_implementations)]
pub struct Pattern;
impl super::Keyword for Pattern {
    fn compile(&self, def: &json::Json, ctx: &schema::WalkContext) -> super::KeywordResult {
        let pattern = keyword_key_exists!(def, "pattern");

        if pattern.is_string() {
            let pattern_val = pattern.as_string().unwrap();
            match regex::Regex::new(pattern_val) {
                Ok(re) => Ok(Some(Box::new(validators::Pattern {
                    regex: re
                }))),
                Err(err) => Err(schema::SchemaError::Malformed {
                    path: ctx.fragment.connect("/"),
                    detail: format!("The value of pattern MUST be a valid RegExp, but {:?}", err)
                }),
            }
        } else {
            Err(schema::SchemaError::Malformed {
                path: ctx.fragment.connect("/"),
                detail: "The value of pattern MUST be a string".to_string()
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
        s.pattern(r"abb.*");
    }).into_json()).ok().unwrap();

    assert_eq!(schema.validate(&"abb".to_json()).is_valid(), true);
    assert_eq!(schema.validate(&"abbd".to_json()).is_valid(), true);
    assert_eq!(schema.validate(&"abd".to_json()).is_valid(), false);
}

#[test]
fn mailformed() {
    let mut scope = scope::Scope::new();

    assert!(scope.compile_and_return(jsonway::object(|schema| {
        schema.set("pattern", "([]".to_string());
    }).unwrap()).is_err());

    assert!(scope.compile_and_return(jsonway::object(|schema| {
        schema.set("pattern", 2);
    }).unwrap()).is_err());
}