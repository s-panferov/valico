use rustc_serialize::json;
use std::collections;

use super::super::schema;

type FormatBuilders = collections::HashMap<String, Box<super::Keyword + Send + Sync>>;

#[allow(missing_copy_implementations)]
pub struct Format {
    formats: FormatBuilders
}

fn default_formats() -> FormatBuilders  {
    let map = collections::HashMap::new();
    map
}

impl Format {
    pub fn new() -> Format {
        Format {
            formats: default_formats()
        }
    }
}

impl super::Keyword for Format {
    fn compile(&self, def: &json::Json, ctx: &schema::WalkContext) -> super::KeywordResult {
        let format = keyword_key_exists!(def, "format");

        if format.is_string() {
            let format = format.as_string().unwrap();
            match self.formats.get(format) {
                Some(keyword) => {
                    keyword.compile(def, ctx)
                },
                None => {
                    Ok(None)
                }
            }
        } else {
            Err(schema::SchemaError::Malformed {
                path: ctx.fragment.connect("/"),
                detail: "The value of format MUST be a string".to_string()
            })
        }
    }
}