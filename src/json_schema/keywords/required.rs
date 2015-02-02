use serialize::json;

use super::super::schema;
use super::super::validators;

#[allow(missing_copy_implementations)]
pub struct Required;
impl super::Keyword for Required {
    fn compile(&self, def: &json::Json, ctx: &schema::WalkContext) -> super::KeywordResult {
        let required = keyword_key_exists!(def, "required");

        if required.is_array() {
            let required = required.as_array().unwrap();

            if required.len() == 0 {
                return Err(schema::SchemaError::Malformed {
                    path: ctx.fragment.connect("/"),
                    detail: "This array MUST have at least one element.".to_string()
                })
            }

            let mut items = vec![];
            for item in required.iter() {
                if item.is_string() {
                    items.push(item.as_string().unwrap().to_string())
                } else {
                    return Err(schema::SchemaError::Malformed {
                        path: ctx.fragment.connect("/"),
                        detail: "The values of `required` MUST be strings".to_string()
                    })
                }
            }

            Ok(Some(Box::new(validators::Required {
                items: items
            })))
        } else {
            Err(schema::SchemaError::Malformed {
                path: ctx.fragment.connect("/"),
                detail: "The value of this keyword MUST be an array.".to_string()
            })
        }
    }
}

#[cfg(test)] use super::super::scope;
#[cfg(test)] use jsonway;

#[test]
fn validate() {
    let mut scope = scope::Scope::new();
    let schema = scope.compile_and_return(jsonway::object(|schema| {
        schema.array("required", |required| {
            required.push("prop1".to_string());
            required.push("prop2".to_string());
        });
    }).unwrap()).ok().unwrap();

    assert_eq!(schema.validate(&jsonway::object(|obj| {
        obj.set("prop1", 0);
    }).unwrap()).is_valid(), false);

    assert_eq!(schema.validate(&jsonway::object(|obj| {
        obj.set("prop2", 0);
    }).unwrap()).is_valid(), false);

    assert_eq!(schema.validate(&jsonway::object(|obj| {
        obj.set("prop1", 0);
        obj.set("prop2", 0);
    }).unwrap()).is_valid(), true);
}

#[test]
fn malformed() {
    let mut scope = scope::Scope::new();

    assert!(scope.compile_and_return(jsonway::object(|schema| {
        schema.array("required", |_| {});
    }).unwrap()).is_err());

    assert!(scope.compile_and_return(jsonway::object(|schema| {
        schema.array("required", |required| {
            required.push(1)
        });
    }).unwrap()).is_err());
}