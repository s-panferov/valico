use serialize::json;
use std::num::{Float};

use super::super::schema;
use super::super::validators;

#[allow(missing_copy_implementations)]
pub struct MaxLength;
impl super::Keyword for MaxLength {
    fn compile(&self, def: &json::Json, ctx: &schema::WalkContext) -> super::KeywordResult {
        let max_length = keyword_key_exists!(def, "maxLength");
        println!("max_length, {}", max_length);

        if max_length.is_number() {
            let max_lenght_val = max_length.as_f64().unwrap();
            if max_lenght_val >= 0f64 && max_lenght_val.fract() == 0f64 {
                Ok(Some(Box::new(validators::MaxLength {
                    length: max_lenght_val as u64
                })))
            } else {
                Err(schema::SchemaError::Malformed {
                    path: ctx.fragment.connect("/"),
                    detail: "The value of `maxLength` MUST be a positive integer or zero".to_string()
                })  
            }
        } else {
            Err(schema::SchemaError::Malformed {
                path: ctx.fragment.connect("/"),
                detail: "The value of `maxLength` MUST be a positive integer or zero".to_string()
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
        schema.set("maxLength", 5);
    }).unwrap()).ok().unwrap();;

    assert_eq!(schema.validate(&"1234".to_json()).valid, true);
    assert_eq!(schema.validate(&"12345".to_json()).valid, true);
    assert_eq!(schema.validate(&"123456".to_json()).valid, false);
}

#[test]
fn malformed() {
    let mut scope = scope::Scope::new();

    assert!(scope.compile_and_return(jsonway::object(|schema| {
        schema.set("maxLength", (-1).to_json());
    }).unwrap()).is_err());

    assert!(scope.compile_and_return(jsonway::object(|schema| {
        schema.set("maxLength", "".to_json());
    }).unwrap()).is_err());

    assert!(scope.compile_and_return(jsonway::object(|schema| {
        schema.set("maxLength", (1.1).to_json());
    }).unwrap()).is_err());
}