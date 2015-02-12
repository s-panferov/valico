use rustc_serialize::json;

use super::super::schema;
use super::super::validators;
use super::super::helpers;

#[allow(missing_copy_implementations)]
pub struct Not;
impl super::Keyword for Not {
    fn compile(&self, def: &json::Json, ctx: &schema::WalkContext) -> super::KeywordResult {
        let not = keyword_key_exists!(def, "not");

        if not.is_object() {
            Ok(Some(Box::new(validators::Not {
                url: helpers::alter_fragment_path(ctx.url.clone(), [
                        ctx.escaped_fragment().as_slice().as_slice(),
                        "not"
                     ].connect("/"))
            })))
        } else {
            Err(schema::SchemaError::Malformed {
                path: ctx.fragment.connect("/"),
                detail: "The value of `not` MUST be an object".to_string()
            })
        }
    }
}