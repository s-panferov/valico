use serialize::json;

use super::super::schema;
use super::super::validators;

#[allow(missing_copy_implementations)]
pub struct MultipleOf;
impl super::Keyword for MultipleOf {
    fn compile(&self, def: &json::Json, ctx: &schema::WalkContext) -> super::KeywordResult {
        let multiple_of = keyword_key_exists!(def, "multipleOf");
        println!("multiple_of, {}", multiple_of);

        if multiple_of.is_number() {
            let multiple_of = multiple_of.as_f64().unwrap();
            if multiple_of > 0f64 {
                Ok(Some(Box::new(validators::MultipleOf {
                    number: multiple_of
                })))
            } else {
                Err(schema::SchemaError::Malformed {
                    path: ctx.fragment.connect("/"),
                    detail: "The value of multipleOf MUST be strictly greater than 0".to_string()
                })
            }
        } else {
            Err(schema::SchemaError::Malformed {
                path: ctx.fragment.connect("/"),
                detail: "The value of multipleOf MUST be a JSON number".to_string()
            })
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
        schema.set("multipleOf", 3.5);
    }).unwrap()).ok().unwrap();

    assert_eq!(schema.validate(&"".to_json()).valid, true);
    assert_eq!(schema.validate(&7.to_json()).valid, true);
    assert_eq!(schema.validate(&6.to_json()).valid, false);
}