use rustc_serialize::json;

use super::super::schema;
use super::super::validators;

#[allow(missing_copy_implementations)]
pub struct UniqueItems;
impl super::Keyword for UniqueItems {
    fn compile(&self, def: &json::Json, ctx: &schema::WalkContext) -> super::KeywordResult {
        let uniq = keyword_key_exists!(def, "uniqueItems");

        if uniq.is_boolean() {
            if uniq.as_boolean().unwrap() {
                Ok(Some(Box::new(validators::UniqueItems)))
            } else {
                Ok(None)
            }
        } else {
            Err(schema::SchemaError::Malformed {
                path: ctx.fragment.connect("/"),
                detail: "The value of pattern MUST be boolean".to_string()
            })
        }
    }
}

#[cfg(test)] use super::super::scope;
#[cfg(test)] use jsonway;
#[cfg(test)] use rustc_serialize::json::{ToJson};

#[test]
fn validate_unique_items() {
    let mut scope = scope::Scope::new();
    let schema = scope.compile_and_return(jsonway::object(|schema| {
        schema.set("uniqueItems", true);
    }).unwrap()).ok().unwrap();;

    assert_eq!(schema.validate(&[1,2,3,4].to_json()).is_valid(), true);
    assert_eq!(schema.validate(&[1,1,3,4].to_json()).is_valid(), false);
}
