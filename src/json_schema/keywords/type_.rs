use rustc_serialize::json;

use super::super::schema;
use super::super::validators;

#[allow(missing_copy_implementations)]
pub struct Type;
impl super::Keyword for Type {
    fn compile(&self, def: &json::Json, ctx: &schema::WalkContext) -> super::KeywordResult {
        let type_ = keyword_key_exists!(def, "type");

        if type_.is_string() {
            let ty = type_.as_string().unwrap().parse();

            if ty.is_some() {
                Ok(Some(Box::new(validators::Type {
                    item: validators::type_::TypeKind::Single(ty.unwrap())
                })))
            } else {
                Err(schema::SchemaError::Malformed {
                    path: ctx.fragment.connect("/"),
                    detail: format!("String values MUST be one of the seven primitive types defined by the core specification. Unknown type: {}", type_)
                })
            }

        } else if type_.is_array() {
            let types = type_.as_array().unwrap();

            if types.len() == 0 {
                return Err(schema::SchemaError::Malformed {
                    path: ctx.fragment.connect("/"),
                    detail: "This array MUST have at least one element.".to_string()
                })
            }

            let mut converted_types = vec![];
            for ty in types.iter() {
                if ty.is_string() {
                    let converted_ty = ty.as_string().unwrap().parse();
                    if converted_ty.is_some() {
                        converted_types.push(converted_ty.unwrap());
                    } else {
                        return Err(schema::SchemaError::Malformed {
                            path: ctx.fragment.connect("/"),
                            detail: format!("Unknown type: {}", ty)
                        })
                    }
                } else {
                    return Err(schema::SchemaError::Malformed {
                        path: ctx.fragment.connect("/"),
                        detail: "String values MUST be one of the seven primitive types defined by the core specification.".to_string()
                    })
                }
            }

            Ok(Some(Box::new(validators::Type {
                item: validators::type_::TypeKind::Set(converted_types)
            })))
        } else {
            Err(schema::SchemaError::Malformed {
                path: ctx.fragment.connect("/"),
                detail: "The value of this keyword MUST be either a string or an array.".to_string()
            })
        }
    }
}

#[cfg(test)] use super::super::scope;
#[cfg(test)] use jsonway;
#[cfg(test)] use rustc_serialize::json::{ToJson};

// pub enum PrimitiveType {
//     Array,
//     Boolean,
//     Integer,
//     Number,
//     Null,
//     Object,
//     String,
// }

#[test]
fn validate_array() {
    let mut scope = scope::Scope::new();
    let schema = scope.compile_and_return(jsonway::object(|schema| {
        schema.set("type", "array".to_string());
    }).unwrap()).ok().unwrap();

    assert_eq!(schema.validate(&jsonway::array(|_arr| {}).unwrap()).is_valid(), true);
    assert_eq!(schema.validate(&"string".to_json()).is_valid(), false);
}

#[test]
fn validate_boolean() {
    let mut scope = scope::Scope::new();
    let schema = scope.compile_and_return(jsonway::object(|schema| {
        schema.set("type", "boolean".to_string());
    }).unwrap()).ok().unwrap();

    assert_eq!(schema.validate(&true.to_json()).is_valid(), true);
    assert_eq!(schema.validate(&false.to_json()).is_valid(), true);
    assert_eq!(schema.validate(&"string".to_json()).is_valid(), false);
}

#[test]
fn validate_integer() {
    let mut scope = scope::Scope::new();
    let schema = scope.compile_and_return(jsonway::object(|schema| {
        schema.set("type", "integer".to_string());
    }).unwrap()).ok().unwrap();

    assert_eq!(schema.validate(&10.to_json()).is_valid(), true);
    assert_eq!(schema.validate(&(-10).to_json()).is_valid(), true);
    assert_eq!(schema.validate(&(11.5).to_json()).is_valid(), false);
    assert_eq!(schema.validate(&"string".to_json()).is_valid(), false);
}

#[test]
fn validate_number() {
    let mut scope = scope::Scope::new();
    let schema = scope.compile_and_return(jsonway::object(|schema| {
        schema.set("type", "number".to_string());
    }).unwrap()).ok().unwrap();

    assert_eq!(schema.validate(&10.to_json()).is_valid(), true);
    assert_eq!(schema.validate(&(-10).to_json()).is_valid(), true);
    assert_eq!(schema.validate(&(11.5).to_json()).is_valid(), true);
    assert_eq!(schema.validate(&"string".to_json()).is_valid(), false);
}

#[test]
fn validate_null() {
    let mut scope = scope::Scope::new();
    let schema = scope.compile_and_return(jsonway::object(|schema| {
        schema.set("type", "null".to_string());
    }).unwrap()).ok().unwrap();

    assert_eq!(schema.validate(&json::Json::Null).is_valid(), true);
    assert_eq!(schema.validate(&"string".to_json()).is_valid(), false);
}

#[test]
fn validate_object() {
    let mut scope = scope::Scope::new();
    let schema = scope.compile_and_return(jsonway::object(|schema| {
        schema.set("type", "object".to_string());
    }).unwrap()).ok().unwrap();

    assert_eq!(schema.validate(&jsonway::object(|_arr| {}).unwrap()).is_valid(), true);
    assert_eq!(schema.validate(&"string".to_json()).is_valid(), false);
}

#[test]
fn validate_string() {
    let mut scope = scope::Scope::new();
    let schema = scope.compile_and_return(jsonway::object(|schema| {
        schema.set("type", "string".to_string());
    }).unwrap()).ok().unwrap();

    assert_eq!(schema.validate(&"string".to_json()).is_valid(), true);
    assert_eq!(schema.validate(&jsonway::object(|_arr| {}).unwrap()).is_valid(), false);
}

#[test]
fn validate_set() {
    let mut scope = scope::Scope::new();
    let schema = scope.compile_and_return(jsonway::object(|schema| {
        schema.array("type", |types| {
            types.push("integer".to_string());
            types.push("string".to_string());
        });
    }).unwrap()).ok().unwrap();

    assert_eq!(schema.validate(&10.to_json()).is_valid(), true);
    assert_eq!(schema.validate(&(-11).to_json()).is_valid(), true);
    assert_eq!(schema.validate(&"string".to_json()).is_valid(), true);
    assert_eq!(schema.validate(&(11.5).to_json()).is_valid(), false);
    assert_eq!(schema.validate(&jsonway::object(|_arr| {}).unwrap()).is_valid(), false);
}

#[test]
fn malformed() {
    let mut scope = scope::Scope::new();

    assert!(scope.compile_and_return(jsonway::object(|schema| {
        schema.set("type", 10);
    }).unwrap()).is_err());

    assert!(scope.compile_and_return(jsonway::object(|schema| {
        schema.object("type", |_type| {});
    }).unwrap()).is_err());

    assert!(scope.compile_and_return(jsonway::object(|schema| {
        schema.set("type", "unsigned".to_string());
    }).unwrap()).is_err());

    assert!(scope.compile_and_return(jsonway::object(|schema| {
        schema.array("type", |types| {
            types.push(10);
        });
    }).unwrap()).is_err());

    assert!(scope.compile_and_return(jsonway::object(|schema| {
        schema.array("type", |types| {
            types.push("unsigned".to_string());
        });
    }).unwrap()).is_err());
}
