use crate::json_schema::SchemaVersion;

use super::super::helpers;
use super::super::schema;
use super::super::validators;
use super::Keyword;

pub struct UnevaluatedItems;
impl Keyword for UnevaluatedItems {
    fn compile(
        &self,
        def: &serde_json::Value,
        ctx: &crate::json_schema::schema::WalkContext,
    ) -> super::KeywordResult {
        if ctx.version < SchemaVersion::Draft2019_09 {
            return Ok(None);
        }
        let items = keyword_key_exists!(def, "unevaluatedItems");

        let validator = match items {
            serde_json::Value::Bool(bool) => validators::UnevaluatedItems::Bool(*bool),
            serde_json::Value::Object(_) => {
                validators::UnevaluatedItems::Schema(helpers::alter_fragment_path(
                    ctx.url.clone(),
                    [ctx.escaped_fragment().as_ref(), "unevaluatedItems"].join("/"),
                ))
            }
            _ => {
                return Err(schema::SchemaError::Malformed {
                    path: ctx.fragment.join("/"),
                    detail: "unevaluatedItems MUST be a bool or an object".to_string(),
                })
            }
        };

        Ok(Some(Box::new(validator)))
    }

    fn place_last(&self) -> bool {
        true
    }
}
