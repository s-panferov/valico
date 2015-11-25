use rustc_serialize::json;

use super::super::schema;
use super::super::validators;

#[allow(missing_copy_implementations)]
pub struct Ref;
impl super::Keyword for Ref {
    fn compile(&self, def: &json::Json, ctx: &schema::WalkContext) -> super::KeywordResult {
        let ref_ = keyword_key_exists!(def, "$ref");

        if ref_.is_string() {
            let url = url_parser!().base_url(ctx.url).parse(ref_.as_string().unwrap());
            match url {
                Ok(url) => {
                    Ok(Some(Box::new(validators::Ref {
                        url: url
                    })))
                },
                Err(_) => {
                    Err(schema::SchemaError::Malformed {
                        path: ctx.fragment.join("/"),
                        detail: "The value of $ref MUST be an URI-encoded JSON Pointer".to_string()
                    })
                }
            }
        } else {
            Err(schema::SchemaError::Malformed {
                path: ctx.fragment.join("/"),
                detail: "The value of multipleOf MUST be a string".to_string()
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
        s.array();
        s.max_items(2u64);
        s.items_schema(|items| {
            items.ref_("#");
        })
    }).into_json(), true).ok().unwrap();

    let array: Vec<String> = vec![];
    let array2: Vec<Vec<String>> = vec![vec![], vec![]];
    let array3: Vec<Vec<String>> = vec![vec![], vec![], vec![]];

    assert_eq!(schema.validate(&array.to_json()).is_valid(), true);
    assert_eq!(schema.validate(&array2.to_json()).is_valid(), true);

    assert_eq!(schema.validate(&array3.to_json()).is_valid(), false);
    assert_eq!(schema.validate(&vec![1,2].to_json()).is_valid(), false);
}

#[test]
fn malformed() {
    let mut scope = scope::Scope::new();

    assert!(scope.compile_and_return(jsonway::object(|schema| {
        schema.set("$ref", "///".to_string());
    }).unwrap(), true).is_err());
}