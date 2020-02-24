use serde_json::Value;

use super::super::errors;
use super::super::scope;

#[allow(missing_copy_implementations)]
pub struct Contains {
    pub url: url::Url,
}

impl super::Validator for Contains {
    fn validate(&self, val: &Value, path: &str, scope: &scope::Scope) -> super::ValidationState {
        let array = nonstrict_process!(val.as_array(), path);

        let schema = scope.resolve(&self.url);
        let mut state = super::ValidationState::new();

        if let Some(schema) = schema {
            let any_matched = array.iter().enumerate().any(|(idx, item)| {
                let item_path = [path, idx.to_string().as_ref()].join("/");
                schema.validate_in(item, item_path.as_ref()).is_valid()
            });

            if !any_matched {
                state.errors.push(Box::new(errors::Contains {
                    path: path.to_string(),
                }))
            }
        } else {
            state.missing.push(self.url.clone());
        }

        state
    }
}
