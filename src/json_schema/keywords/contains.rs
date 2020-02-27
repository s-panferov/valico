use serde_json::Value;

use super::super::helpers;
use super::super::schema;
use super::super::validators;

#[allow(missing_copy_implementations)]
pub struct Contains;
impl super::Keyword for Contains {
    fn compile(&self, def: &Value, ctx: &schema::WalkContext) -> super::KeywordResult {
        let contains = keyword_key_exists!(def, "contains");

        if contains.is_object() || contains.is_boolean() {
            Ok(Some(Box::new(validators::Contains {
                url: helpers::alter_fragment_path(
                    ctx.url.clone(),
                    [ctx.escaped_fragment().as_ref(), "contains"].join("/"),
                ),
            })))
        } else {
            Err(schema::SchemaError::Malformed {
                path: ctx.fragment.join("/"),
                detail: "The value of contains MUST be an object or a boolean".to_string(),
            })
        }
    }
}
