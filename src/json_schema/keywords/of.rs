use serialize::json;

use super::super::schema;
use super::super::validators;
use super::super::helpers;

macro_rules! of_keyword{
    ($name:ident, $kw:expr) => {

        #[allow(missing_copy_implementations)]
        pub struct $name;
        impl super::Keyword for $name {
            fn compile(&self, def: &json::Json, ctx: &schema::WalkContext) -> super::KeywordResult {
                let of = keyword_key_exists!(def, $kw);

                if of.is_array() {
                    let of = of.as_array().unwrap();

                    if of.len() == 0 {
                        return Err(schema::SchemaError::Malformed {
                            path: ctx.fragment.connect("/"),
                            detail: "This array MUST have at least one element.".to_string()
                        })
                    }

                    let mut schemes = vec![];
                    for (idx, scheme) in of.iter().enumerate() {
                        if scheme.is_object() {
                            schemes.push(
                                helpers::alter_fragment_path(ctx.url.clone(), [
                                    ctx.escaped_fragment().as_slice().as_slice(), 
                                    $kw,
                                    idx.to_string().as_slice()
                                ].connect("/"))
                            )
                        } else {
                            return Err(schema::SchemaError::Malformed {
                                path: ctx.fragment.connect("/"),
                                detail: "Elements of the array MUST be objects.".to_string()
                            })
                        }
                    }

                    Ok(Some(Box::new(validators::$name {
                        schemes: schemes
                    })))
                } else {
                    Err(schema::SchemaError::Malformed {
                        path: ctx.fragment.connect("/"),
                        detail: "The value of this keyword MUST be an array.".to_string()
                    })
                }
            }
        }

    }
}

of_keyword!(AllOf, "allOf");
of_keyword!(AnyOf, "anyOf");
of_keyword!(OneOf, "oneOf");

#[cfg(test)] use super::super::scope;
#[cfg(test)] use jsonway;
#[cfg(test)] use serialize::json::{ToJson};

#[test]
fn validate_all_of() {
    let mut scope = scope::Scope::new();
    let schema = scope.compile_and_return(jsonway::object(|schema| {
        schema.array("allOf", |all_of| {
            all_of.object(|schema| {
                schema.set("minimum", 5);
            });
            all_of.object(|schema| {
                schema.set("maximum", 10);
            });
        });
    }).unwrap()).ok().unwrap();

    assert_eq!(schema.validate(&7.to_json()).is_valid(), true);
    assert_eq!(schema.validate(&4.to_json()).is_valid(), false);
    assert_eq!(schema.validate(&11.to_json()).is_valid(), false);
}

#[test]
fn validate_any_of() {
    let mut scope = scope::Scope::new();
    let schema = scope.compile_and_return(jsonway::object(|schema| {
        schema.array("anyOf", |all_of| {
            all_of.object(|schema| {
                schema.set("maximum", 5);
            });
            all_of.object(|schema| {
                schema.set("maximum", 10);
            });
        });
    }).unwrap()).ok().unwrap();

    assert_eq!(schema.validate(&5.to_json()).is_valid(), true);
    assert_eq!(schema.validate(&10.to_json()).is_valid(), true);
    assert_eq!(schema.validate(&11.to_json()).is_valid(), false);
}

#[test]
fn validate_one_of() {
    let mut scope = scope::Scope::new();
    let schema = scope.compile_and_return(jsonway::object(|schema| {
        schema.array("oneOf", |all_of| {
            all_of.object(|schema| {
                schema.set("maximum", 5);
            });
            all_of.object(|schema| {
                schema.set("maximum", 10);
            });
        });
    }).unwrap()).ok().unwrap();

    assert_eq!(schema.validate(&5.to_json()).is_valid(), false);
    assert_eq!(schema.validate(&6.to_json()).is_valid(), true);
    assert_eq!(schema.validate(&11.to_json()).is_valid(), false);
}

